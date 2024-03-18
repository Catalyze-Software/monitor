use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use ic_ledger_types::Tokens;
use ic_stable_structures::{storable::Bound, Storable};

use crate::operations::canisters::sns::{CanisterSummary, GetSnsCanistersSummaryResponse};

pub type Timestamp = u64;

/*
* Models
*/
#[derive(CandidType, Deserialize, Clone)]
pub struct Log {
    pub timestamp: Timestamp,
    pub msg: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MonitorData {
    pub timestamp: Timestamp,
    pub icp_balance: Tokens,
    pub cycle_balance: Nat,
}

pub type SnsData = GetSnsCanistersSummaryResponse;

// 6 child canisters
#[derive(CandidType, Deserialize, Clone)]
pub struct ChildData {
    pub timestamp: Timestamp,
    pub members: CanisterCycles,
    pub groups: CanisterCycles,
    pub profiles: CanisterCycles,
    pub events: CanisterCycles,
    pub event_attendees: CanisterCycles,
    pub reports: CanisterCycles,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct FrontendData {
    pub timestamp: Timestamp,
    pub frontend: CanisterCycles,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SiweData {
    pub timestamp: Timestamp,
    pub siwe: CanisterCycles,
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

impl Storable for FrontendData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode FrontendData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode FrontendData")
    }
}

impl Storable for SiweData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode SIWEData"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).expect("Failed to decode SIWEData")
    }
}

/*
* CanisterCycles model
*/
#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterCycles {
    pub name: String,
    pub canister_id: Principal,
    pub cycles: Nat,
}

impl CanisterCycles {
    pub fn new<'a>(name: &str, canister_summary: &CanisterSummary) -> Self {
        Self {
            name: String::from(name),
            canister_id: canister_summary.canister_id.unwrap(),
            cycles: canister_summary.status.as_ref().unwrap().cycles.clone(),
        }
    }

    pub fn from_status<'a>(name: &str, canister_id: &str, status: &CanisterStatusResponse) -> Self {
        Self {
            name: String::from(name),
            canister_id: Principal::from_text(canister_id).expect("Invalid principal"),
            cycles: Nat::from(status.cycles.clone()),
        }
    }
}
