use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Nat};
use ic_cdk::api::management_canister::main::{
    CanisterId, CanisterStatusType, DefiniteCanisterSettings, QueryStats,
};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

type Timestamp = u64;

#[derive(CandidType, Deserialize, Clone)]
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
    pub status: CanisterStatusType,
    pub settings: DefiniteCanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub memory_size: Nat,
    pub cycles: Nat,
    pub idle_cycles_burned_per_day: Nat,
    pub query_stats: Option<QueryStats>,
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
