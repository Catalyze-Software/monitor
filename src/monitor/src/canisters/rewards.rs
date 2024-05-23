use candid::{CandidType, Principal};
use serde::Deserialize;

const REWARDS_CANISTER_PRINCIPAL: &str = "zgfl7-pqaaa-aaaap-accpa-cai";

#[derive(Deserialize, CandidType, Clone)]
pub struct RewardData {
    pub timestamp: u64,
    pub principal: Principal,
    pub description: String,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct GroupInfo {
    owner: Principal,
    count_milestone: u64,
    activity_milestone: u64,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct EventInfo {
    owner: Principal,
    attendance_milestone: u64,
}

pub async fn group_info() -> Vec<GroupInfo> {
    ic_cdk::call::<(), (Vec<GroupInfo>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "group_info",
        (),
    )
    .await
    .expect("Failed to call group_info")
    .0
}

pub async fn event_info() -> Vec<EventInfo> {
    ic_cdk::call::<(), (Vec<EventInfo>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "event_info",
        (),
    )
    .await
    .expect("Failed to call event_info")
    .0
}

pub async fn token_balances() -> Vec<(Principal, u64)> {
    ic_cdk::call::<(), (Vec<(Principal, u64)>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "token_balances",
        (),
    )
    .await
    .expect("Failed to call token_balances")
    .0
}

pub async fn log_size() -> u64 {
    ic_cdk::call::<(), (u64,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "log_size",
        (),
    )
    .await
    .expect("Failed to call log_size")
    .0
}

pub async fn graph_member_count_rewards() -> Vec<(u64, u64)> {
    ic_cdk::call::<(), (Vec<(u64, u64)>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "graph_member_count_rewards",
        (),
    )
    .await
    .expect("Failed to call graph_member_count_rewards")
    .0
}

pub async fn graph_member_activity_rewards() -> Vec<(u64, u64)> {
    ic_cdk::call::<(), (Vec<(u64, u64)>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "graph_member_activity_rewards",
        (),
    )
    .await
    .expect("Failed to call graph_activity_rewards")
    .0
}

pub async fn graph_event_attendee_rewards() -> Vec<(u64, u64)> {
    ic_cdk::call::<(), (Vec<(u64, u64)>,)>(
        Principal::from_text(REWARDS_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "graph_event_attendee_rewards",
        (),
    )
    .await
    .expect("Failed to call graph_event_attendee_rewards")
    .0
}
