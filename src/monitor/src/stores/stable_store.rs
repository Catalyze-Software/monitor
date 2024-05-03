use super::{
    types::Snapshot,
    types::{Log, MonitorICPBalance},
};
use crate::{queries::range, utils::log::format_time};
use ic_cdk::api::time;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl,
};
use std::cell::RefCell;

/*
* Memory IDs
*/
const MEM_ID_LOGS: MemoryId = MemoryId::new(0);
const MEM_ID_MONITOR: MemoryId = MemoryId::new(1);
const MEM_ID_CANISTER_STATUS_HISTORY: MemoryId = MemoryId::new(254);

/*
* Stable stores
*/
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static LOGS: RefCell<StableBTreeMap<u64, Log, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_LOGS))));

    static CANISTER_STATUS_HISTORY: RefCell<StableBTreeMap<u64, Snapshot, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_CANISTER_STATUS_HISTORY))));

    static MONITOR_STORE: RefCell<StableBTreeMap<u64, MonitorICPBalance, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_MONITOR))));
}

/*
* Unit structs for stable stores
*/
pub struct Logs;
pub struct CanisterStatusStore;
pub struct MonitorStore;

/*
* Imple stable stores
*/
impl Logs {
    pub fn size() -> u64 {
        LOGS.with(|l| l.borrow().len())
    }

    pub fn new_index() -> u64 {
        LOGS.with(|l| l.borrow().last_key_value().map_or(1, |(key, _)| key + 1))
    }

    fn insert(index: u64, log: Log) {
        LOGS.with(|l| l.borrow_mut().insert(index, log));
    }

    pub fn log(log: String) {
        let index = Self::new_index();
        let now = time();

        let log = Log {
            timestamp: now,
            msg: log,
        };

        Self::insert(index, log);
    }

    pub fn get_latest(n: u64) -> Vec<Log> {
        let len = Self::size();

        let (start, end) = range(n, len);

        LOGS.with(|l| {
            l.borrow()
                .range(start..=end)
                .map(|(_, log)| log.clone())
                .collect::<Vec<Log>>()
                .into_iter()
                .rev()
                .collect()
        })
    }

    pub fn get_latest_with_timestamps(n: u64) -> Vec<String> {
        let len = Self::size();

        let (start, end) = range(n, len);

        LOGS.with(|l| {
            l.borrow()
                .range(start..=end)
                .map(|(_, log)| format!("{} {}", format_time(log.timestamp), log.msg.clone()))
                .collect()
        })
    }
}

impl CanisterStatusStore {
    pub fn size() -> u64 {
        CANISTER_STATUS_HISTORY.with(|c| c.borrow().len())
    }

    pub fn new_index() -> u64 {
        CANISTER_STATUS_HISTORY.with(|c| c.borrow().last_key_value().map_or(1, |(key, _)| key + 1))
    }

    pub fn insert(snapshot: Snapshot) {
        let index = Self::new_index();

        CANISTER_STATUS_HISTORY.with(|c| c.borrow_mut().insert(index, snapshot));
    }

    pub fn get_latest() -> Option<Snapshot> {
        let (_, value) = CANISTER_STATUS_HISTORY
            .with(|c| c.borrow().last_key_value().expect("No canister data"));
        Some(value.clone())
    }

    pub fn get_latest_n(n: u64) -> Vec<Snapshot> {
        let len = Self::size();

        let (start, end) = range(n, len);

        CANISTER_STATUS_HISTORY.with(|c| {
            c.borrow()
                .range(start..=end)
                .map(|(_, snapshot)| snapshot.clone())
                .collect()
        })
    }
}

impl MonitorStore {
    pub fn size() -> u64 {
        MONITOR_STORE.with(|m| m.borrow().len())
    }

    pub fn new_index() -> u64 {
        MONITOR_STORE.with(|m| m.borrow().last_key_value().map_or(1, |(key, _)| key + 1))
    }

    pub fn insert(monitor_data: MonitorICPBalance) {
        let index = Self::new_index();

        MONITOR_STORE.with(|m| m.borrow_mut().insert(index, monitor_data));
    }

    pub fn get_latest() -> Option<MonitorICPBalance> {
        let (_, value) =
            MONITOR_STORE.with(|m| m.borrow().last_key_value().expect("No monitor data"));
        Some(value.clone())
    }

    pub fn get_latest_n(n: u64) -> Vec<MonitorICPBalance> {
        let len = Self::size();

        let (start, end) = range(n, len);

        MONITOR_STORE.with(|m| {
            m.borrow()
                .range(start..=end)
                .map(|(_, monitor_data)| monitor_data.clone())
                .collect()
        })
    }
}
