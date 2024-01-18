use crate::{
    operations::{
        child::get_child_canister_summary,
        cmc::{icp_xdr_rate, notify_top_up},
        ledger::transfer_icp_to_cmc_for_cycles_minting,
        sns::GetSnsCanistersSummaryResponse,
    },
    store::CanisterCycles,
};
use candid::{CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::{api::time, trap};
use ic_ledger_types::Tokens;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, Storable,
};
use std::{borrow::Cow, cell::RefCell};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

/*
* Type aliases
*/
type Timestamp = u64;

/*
* Models
*/
#[derive(CandidType, Deserialize, Clone)]
struct Log {
    pub timestamp: Timestamp,
    pub msg: String,
}

#[derive(CandidType, Deserialize, Clone)]
struct MonitorData {
    icp_balance: Tokens,
    cycle_balance: Nat,
}

type SnsData = GetSnsCanistersSummaryResponse;

// 6 child canisters
#[derive(CandidType, Deserialize, Clone)]
struct ChildData {
    pub members: CanisterCycles,
    pub groups: CanisterCycles,
    pub profiles: CanisterCycles,
    pub events: CanisterCycles,
    pub event_attendees: CanisterCycles,
    pub reports: CanisterCycles,
}

/*
* Impl Storable for Models
*/
impl Storable for Log {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode Log"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode Log")
    }
}

impl Storable for MonitorData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode MonitorData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode MonitorData")
    }
}

impl Storable for SnsData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode SnsData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode SnsData")
    }
}

impl Storable for ChildData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode ChildData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode ChildData")
    }
}

/*
* Memory IDs
*/
const MEM_ID_LOGS: MemoryId = MemoryId::new(0);

const MEM_ID_MONITOR: MemoryId = MemoryId::new(1);
const MEM_ID_SNS_CANISTERS: MemoryId = MemoryId::new(2);
const MEM_ID_CHILD_CANISTERS: MemoryId = MemoryId::new(3);

// const MEM_ID_HEAP_STORE: MemoryId = MemoryId::new(4);

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

    // static HEAP_STORE: RefCell<StableCell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
    //     StableCell::init(
    //         MEMORY_MANAGER.with(|mm| mm.borrow().get(MEM_ID_HEAP_STORE)),
    //         Vec::new(),
    //     ).expect("Failed to init HEAP_STORE"));
}

/*
* Unit structs for stable stores
*/
pub struct Logs;

struct MonitorStore;
struct SnsStore;
struct ChildStore;

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

/*
* Perform canister status query routine
*/
const EVENT_MONITOR_DATA: &str = "Fetched and stored monitor data";
const EVENT_SNS_DATA: &str = "Fetched and stored SNS canisters summary";
const EVENT_CHILD_SUMMARY: &str = "Fetched and stored child canisters summary";
const EVENT_ICP_SENT: &str = "ICP sent";
const EVENT_CYCLES_MINTED: &str = "Cycles minted";

pub async fn read_operations() {
    // Monitor read, store and log operations
    let icp_balance = crate::operations::ledger::icp_balance().await;
    let cycle_balance = crate::operations::cmc::cycle_balance().await;

    let monitor_data = MonitorData {
        icp_balance,
        cycle_balance,
    };

    MonitorStore::insert(monitor_data);
    Logs::log(format!("{}", EVENT_MONITOR_DATA.to_string()));

    // SNS read, store and log operations
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    SnsStore::insert(summary);
    Logs::log(format!("{}", EVENT_SNS_DATA.to_string()));

    // Child read, store and log operations
    let childs = get_child_canister_summary().await;
    let child_data = ChildData {
        members: childs[0].clone(),
        groups: childs[1].clone(),
        profiles: childs[2].clone(),
        events: childs[3].clone(),
        event_attendees: childs[4].clone(),
        reports: childs[5].clone(),
    };

    ChildStore::insert(child_data);
    Logs::log(format!("{}", EVENT_CHILD_SUMMARY.to_string()));
}

/*
* Sorted canister cycles
*/
pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    // add this monitor canister
    let monitor_data = MonitorStore::get_latest().unwrap();

    vec.push(CanisterCycles {
        name: String::from("monitor"),
        canister_id: ic_cdk::id(),
        cycles: monitor_data.cycle_balance,
    });

    // add the SNS canisters
    let summary = SnsStore::get_latest().unwrap();

    vec.push(CanisterCycles::new("root", &summary.root.unwrap()));
    vec.push(CanisterCycles::new("swap", &summary.swap.unwrap()));
    vec.push(CanisterCycles::new("ledger", &summary.ledger.unwrap()));
    vec.push(CanisterCycles::new("index", &summary.index.unwrap()));
    vec.push(CanisterCycles::new(
        "governance",
        &summary.governance.unwrap(),
    ));

    // iterate over the dapps canisters
    for (i, canister) in summary.dapps.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("dapps {}", i), &canister));
    }

    // iterate over the archives canisters
    for (i, canister) in summary.archives.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("archives {}", i), &canister));
    }

    // add the child canisters
    let child_data = ChildStore::get_latest().unwrap();

    vec.push(child_data.members.clone());
    vec.push(child_data.groups.clone());
    vec.push(child_data.profiles.clone());
    vec.push(child_data.events.clone());
    vec.push(child_data.event_attendees.clone());
    vec.push(child_data.reports.clone());

    // sort the vec by cycles in ascending order
    vec.sort_by(|a, b| a.cycles.cmp(&b.cycles));

    vec
}

const CYCLES_BALANCE_THRESHOLD: u64 = 6_500_000_000_000; // 5T
const CYCLE_TOP_UP_AMOUNT: u64 = 1_000_000_000;

/*
* Iterate over all canister-cycles vector and top up canisters with low cycles
*/
pub async fn top_up_canisters() {
    let sorted_canister_cycles = sorted_canister_cycles();

    for CanisterCycles {
        name,
        canister_id,
        cycles,
    } in sorted_canister_cycles
    {
        if cycles < CYCLES_BALANCE_THRESHOLD {
            top_up(name, canister_id).await;
        // since this vector is sorted in ascending cycle order, we can break early
        } else {
            break;
        }
    }
}

/*
* Perform all async operations for topping up one canister with cycles and log results
*/
async fn top_up(name: String, canister_id: Principal) {
    // get rate, calculate icp amount and log result
    let rate = icp_xdr_rate().await.xdr_permyriad_per_icp;
    let icp_e8s = CYCLE_TOP_UP_AMOUNT / rate;

    Logs::log(format!(
        "ICP/XDR rate: {}, top up amount: {} ICP",
        rate,
        Tokens::from_e8s(icp_e8s)
    ));

    // transfer icp to CMC for cycles minting and log result
    let block_index =
        transfer_icp_to_cmc_for_cycles_minting(Tokens::from_e8s(icp_e8s), canister_id).await;

    Logs::log(format!(
        "{}: {} ICP",
        EVENT_ICP_SENT,
        Tokens::from_e8s(icp_e8s)
    ));

    // notify CMC of the ICP tx and log result
    let cycles = notify_top_up(block_index, canister_id).await;

    Logs::log(format!(
        "{}: {} cycles for canister '{}' with canister_id {}",
        EVENT_CYCLES_MINTED, cycles, name, canister_id
    ));
}

pub fn format_time(timestamp: u64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128)
        .unwrap_or_else(|e| trap(&format!("Error converting timestamp: {}", e)));

    datetime
        .format(&Rfc2822)
        .unwrap_or_else(|e| trap(&format!("Error formatting datetime: {}", e)))
}
