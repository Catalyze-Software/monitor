use crate::store::STATE;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::call;

use crate::principals::SNS_ROOT;

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

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.summary = summary.clone();
    });

    summary
}
