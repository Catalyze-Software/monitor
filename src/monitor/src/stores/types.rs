use candid::{CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::api::management_canister::main::{
    CanisterId, CanisterStatusType, DefiniteCanisterSettings, QueryStats,
};
use ic_ledger_types::Tokens;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

pub type Timestamp = u64;

#[derive(CandidType, Deserialize, Clone, Default)]
pub struct Snapshot {
    pub timestamp: Timestamp,
    pub canisters: Vec<CanisterSnapshot>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterSnapshot {
    pub canister_name: String,
    pub canister_id: CanisterId,
    pub status: CatalyzeCanisterStatus,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CatalyzeCanisterStatus {
    pub status: Option<CanisterStatusType>,
    pub settings: Option<DefiniteCanisterSettings>,
    pub module_hash: Option<Vec<u8>>,
    pub memory_size: Option<Nat>,
    pub cycles: Option<Nat>,
    pub idle_cycles_burned_per_day: Option<Nat>,
    pub query_stats: Option<QueryStats>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Log {
    pub timestamp: Timestamp,
    pub msg: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MonitorICPBalance {
    pub timestamp: Timestamp,
    pub icp_balance: Tokens,
}

type CanisterName = String;
type TCycles = f64;

#[derive(CandidType, Deserialize)]
pub struct CycleHistory {
    pub timestamps: Vec<Timestamp>,
    pub line_data: Vec<LineData>,
}

#[derive(CandidType, Deserialize)]
pub struct LineData {
    pub canister_name: CanisterName,
    pub cycles: Vec<TCycles>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterCycles {
    pub name: String,
    pub canister_id: Principal,
    pub cycles: Nat,
}

impl From<CanisterSnapshot> for CanisterCycles {
    fn from(value: CanisterSnapshot) -> Self {
        Self {
            name: value.canister_name,
            canister_id: value.canister_id,
            cycles: value.status.cycles.unwrap_or(Nat::from(0)),
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterMemorySize {
    pub name: String,
    pub canister_id: Principal,
    pub size: Nat,
}

impl From<CanisterSnapshot> for CanisterMemorySize {
    fn from(value: CanisterSnapshot) -> Self {
        Self {
            name: value.canister_name,
            canister_id: value.canister_id,
            size: value.status.memory_size.expect("No memory size"),
        }
    }
}

impl Storable for Log {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode Log"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode Log")
    }
}

impl Storable for MonitorICPBalance {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode MonitorData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode MonitorData")
    }
}

impl Storable for Snapshot {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode Snapshot"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Snapshot).expect("Failed to decode Snapshot")
    }

    const BOUND: Bound = Bound::Unbounded;
}
