use super::stable_models::{ChildData, Log, MonitorData, SnsData, Timestamp};
use crate::utils::log::format_time;
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
const MEM_ID_SNS_CANISTERS: MemoryId = MemoryId::new(2);
const MEM_ID_CHILD_CANISTERS: MemoryId = MemoryId::new(3);

/*
* Stable stores
*/
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static LOGS: RefCell<StableBTreeMap<Timestamp, Log, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_LOGS))));

    static MONITOR_STORE: RefCell<StableBTreeMap<u64, MonitorData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_MONITOR))));

    static SNS_STORE: RefCell<StableBTreeMap<u64, SnsData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_SNS_CANISTERS))));

    static CHILD_DATA: RefCell<StableBTreeMap<u64, ChildData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_CHILD_CANISTERS))));
}

/*
* Unit structs for stable stores
*/
pub struct Logs;

pub struct MonitorStore;
pub struct SnsStore;
pub struct ChildStore;

/*
* Imple stable stores
*/
impl Logs {
    pub fn log(log: String) {
        let index = LOGS.with(|l| l.borrow_mut().len() + 1);
        let now = time();

        let log = Log {
            timestamp: now,
            msg: log,
        };

        LOGS.with(|l| l.borrow_mut().insert(index, log));
    }

    pub fn _get(i: u64) -> Option<String> {
        let msg = LOGS.with(|l| l.borrow().get(&i).expect("No logs").msg.clone());
        Some(msg)
    }

    pub fn get_range(start: u64, end: u64) -> Vec<String> {
        LOGS.with(|l| {
            l.borrow()
                .range(start..end)
                .map(|(_, log)| log.msg.clone())
                .collect()
        })
    }

    pub fn get_latest(n: u64) -> Vec<String> {
        let (key, _) = LOGS.with(|l| l.borrow().last_key_value().expect("No logs"));

        // check length of tree to avoid underflow
        if key < n {
            return Logs::get_range(0, key);
        } else {
            return Logs::get_range(key - n, key);
        };
    }

    pub fn get_latest_with_timestamps(n: u64) -> Vec<String> {
        let (key, _) = LOGS.with(|l| l.borrow().last_key_value().expect("No logs"));

        // check length of tree to avoid underflow
        let (start, end) = if key < n { (0, key) } else { (key - n, key) };

        LOGS.with(|l| {
            l.borrow()
                .range(start..end)
                .map(|(_, log)| format!("{} {}", format_time(log.timestamp), log.msg.clone()))
                .collect()
        })
    }
}

impl MonitorStore {
    pub fn insert(monitor_data: MonitorData) {
        let index = MONITOR_STORE.with(|m| m.borrow_mut().len() + 1);

        MONITOR_STORE.with(|m| m.borrow_mut().insert(index, monitor_data));
    }

    pub fn _get(i: u64) -> Option<MonitorData> {
        MONITOR_STORE.with(|m| m.borrow().get(&i))
    }

    pub fn get_latest() -> Option<MonitorData> {
        let (_, value) =
            MONITOR_STORE.with(|m| m.borrow().last_key_value().expect("No monitor data"));
        Some(value.clone())
    }
}

impl SnsStore {
    pub fn insert(sns_data: SnsData) {
        let index = SNS_STORE.with(|s| s.borrow_mut().len() + 1);

        SNS_STORE.with(|s| s.borrow_mut().insert(index, sns_data));
    }

    pub fn _get(i: u64) -> Option<SnsData> {
        SNS_STORE.with(|s| s.borrow().get(&i))
    }

    pub fn get_latest() -> Option<SnsData> {
        let (_, value) = SNS_STORE.with(|s| s.borrow().last_key_value().expect("No SNS data"));
        Some(value.clone())
    }
}

impl ChildStore {
    pub fn insert(child_data: ChildData) {
        let index = CHILD_DATA.with(|c| c.borrow_mut().len() + 1);

        CHILD_DATA.with(|c| c.borrow_mut().insert(index, child_data));
    }

    pub fn _get(i: u64) -> Option<ChildData> {
        CHILD_DATA.with(|c| c.borrow().get(&i))
    }

    pub fn get_latest() -> Option<ChildData> {
        let (_, value) = CHILD_DATA.with(|c| c.borrow().last_key_value().expect("No child data"));
        Some(value.clone())
    }
}
