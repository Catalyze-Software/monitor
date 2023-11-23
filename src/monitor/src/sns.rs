use crate::principals::CHILD_MEMBERS;
use crate::{principals::SNS_ROOT, store::STATE};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::management_canister::main::canister_status;
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;
use ic_cdk::{call, trap};

#[derive(CandidType, Deserialize)]
pub struct GetSnsCanistersSummaryRequest {
    pub update_canister_list: Option<bool>,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct GetSnsCanistersSummaryResponse {
    pub root: Option<CanisterSummary>,
    pub swap: Option<CanisterSummary>,
    pub ledger: Option<CanisterSummary>,
    pub index: Option<CanisterSummary>,
    pub governance: Option<CanisterSummary>,
    pub dapps: Vec<CanisterSummary>,
    pub archives: Vec<CanisterSummary>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterSummary {
    pub status: Option<CanisterStatusResultV2>,
    pub canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterStatusResultV2 {
    pub status: CanisterStatusType,
    pub memory_size: candid::Nat,
    pub cycles: candid::Nat,
    pub settings: DefiniteCanisterSettingsArgs,
    pub idle_cycles_burned_per_day: candid::Nat,
    pub module_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum CanisterStatusType {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "running")]
    Running,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct DefiniteCanisterSettingsArgs {
    pub freezing_threshold: candid::Nat,
    pub controllers: Vec<Principal>,
    pub memory_allocation: candid::Nat,
    pub compute_allocation: candid::Nat,
}

pub async fn get_sns_canisters_summary() -> GetSnsCanistersSummaryResponse {
    let arg = GetSnsCanistersSummaryRequest {
        update_canister_list: None,
    };

    let canister = Principal::from_text(SNS_ROOT).expect("Failed to parse SNS_ROOT");

    let (summary,): (GetSnsCanistersSummaryResponse,) =
        call::<(GetSnsCanistersSummaryRequest,), (GetSnsCanistersSummaryResponse,)>(
            canister,
            "get_sns_canisters_summary",
            (arg,),
        )
        .await
        .expect("Failed to call get_sns_canisters_summary");

    summary
}

/*
* Select each canister-cycles pair from the stored `GetSnsCanistersSummaryResponse`
* and return a sorted vector of these pairs
*/
#[derive(CandidType, Deserialize)]
pub struct CanisterCycles(String, Nat);

pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    let summary = STATE.with(|s| s.borrow().get_summary());

    // get cycles for the general sns canisters
    vec.push(CanisterCycles(
        String::from("root"),
        summary
            .root
            .unwrap_or_else(|| trap("Root canister not found"))
            .status
            .unwrap_or_else(|| trap("Root canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("swap"),
        summary
            .swap
            .unwrap_or_else(|| trap("Swap canister not found"))
            .status
            .unwrap_or_else(|| trap("Swap canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("ledger"),
        summary
            .ledger
            .unwrap_or_else(|| trap("Ledger canister not found"))
            .status
            .unwrap_or_else(|| trap("Ledger canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("index"),
        summary
            .index
            .unwrap_or_else(|| trap("Index canister not found"))
            .status
            .unwrap_or_else(|| trap("Index canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("governance"),
        summary
            .governance
            .unwrap_or_else(|| trap("Governance canister not found"))
            .status
            .unwrap_or_else(|| trap("Governance canister status not found"))
            .cycles,
    ));

    // iterate over the dapps canisters
    for (i, canister) in summary.dapps.iter().enumerate() {
        vec.push(CanisterCycles(
            format!("dapps {}", i),
            canister
                .status
                .as_ref()
                .unwrap_or_else(|| trap("Dapps canister status not found"))
                .cycles
                .clone(),
        ));
    }

    // iterate over the archives canisters
    for (i, canister) in summary.archives.iter().enumerate() {
        vec.push(CanisterCycles(
            format!("archives {}", i),
            canister
                .status
                .as_ref()
                .unwrap_or_else(|| trap("Archives canister status not found"))
                .cycles
                .clone(),
        ));
    }

    // iterate over child canisters (not present in `GetSnsCanistersSummaryResponse`)
    STATE.with(|s| match s.borrow().childs.clone() {
        None => {}
        Some(childs) => {
            for (canister, status) in childs {
                vec.push(CanisterCycles(canister, status.cycles));
            }
        }
    });

    // sort the vec by cycles in ascending order
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec
}
