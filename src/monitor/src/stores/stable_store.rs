use super::stable_models::{ChildData, Log, MonitorData, SnsData};
use crate::{queries::range, utils::log::format_time};
use ic_cdk::api::time;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, Storable,
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

    static LOGS: RefCell<StableBTreeMap<u64, Log, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_LOGS))));

    static MONITOR_STORE: RefCell<StableBTreeMap<u64, MonitorData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_MONITOR))));

    static SNS_STORE: RefCell<StableBTreeMap<u64, SnsData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_SNS_CANISTERS))));

    static CHILD_DATA: RefCell<StableBTreeMap<u64, ChildData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_CHILD_CANISTERS))));
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
    pub fn size() -> u64 {
        LOGS.with(|l| l.borrow().len())
    }

    pub fn new_index() -> u64 {
        LOGS.with(|l| new_index(&l.borrow()))
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

impl MonitorStore {
    pub fn size() -> u64 {
        MONITOR_STORE.with(|m| m.borrow().len())
    }

    pub fn new_index() -> u64 {
        MONITOR_STORE.with(|m| new_index(&m.borrow()))
    }

    pub fn insert(monitor_data: MonitorData) {
        let index = Self::new_index();

        MONITOR_STORE.with(|m| m.borrow_mut().insert(index, monitor_data));
    }

    pub fn get_latest() -> Option<MonitorData> {
        let (_, value) =
            MONITOR_STORE.with(|m| m.borrow().last_key_value().expect("No monitor data"));
        Some(value.clone())
    }

    pub fn get_latest_n(n: u64) -> Vec<MonitorData> {
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

impl SnsStore {
    pub fn size() -> u64 {
        SNS_STORE.with(|s| s.borrow().len())
    }

    pub fn new_index() -> u64 {
        SNS_STORE.with(|s| new_index(&s.borrow()))
    }

    pub fn insert(sns_data: SnsData) {
        let index = Self::new_index();

        SNS_STORE.with(|s| s.borrow_mut().insert(index, sns_data));
    }

    pub fn get_latest() -> Option<SnsData> {
        let (_, value) = SNS_STORE.with(|s| s.borrow().last_key_value().expect("No SNS data"));
        Some(value.clone())
    }

    pub fn get_latest_n(n: u64) -> Vec<SnsData> {
        let len = Self::size();

        let (start, end) = range(n, len);

        SNS_STORE.with(|s| {
            s.borrow()
                .range(start..=end)
                .map(|(_, sns_data)| sns_data.clone())
                .collect()
        })
    }
}

impl ChildStore {
    pub fn size() -> u64 {
        CHILD_DATA.with(|c| c.borrow().len())
    }

    pub fn new_index() -> u64 {
        CHILD_DATA.with(|c| new_index(&c.borrow()))
    }

    pub fn insert(child_data: ChildData) {
        let index = Self::new_index();

        CHILD_DATA.with(|c| c.borrow_mut().insert(index, child_data));
    }

    pub fn get_latest() -> Option<ChildData> {
        let (_, value) = CHILD_DATA.with(|c| c.borrow().last_key_value().expect("No child data"));
        Some(value.clone())
    }

    pub fn get_latest_n(n: u64) -> Vec<ChildData> {
        let len = Self::size();

        let (start, end) = range(n, len);

        CHILD_DATA.with(|c| {
            c.borrow()
                .range(start..=end)
                .map(|(_, child_data)| child_data.clone())
                .collect()
        })
    }
}

fn new_index<Value>(tree: &StableBTreeMap<u64, Value, VirtualMemory<DefaultMemoryImpl>>) -> u64
where
    Value: Storable,
{
    tree.last_key_value().map_or(1, |(key, _)| key + 1)
}
