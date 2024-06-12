use crate::stores::types::Timestamp;
use candid::{CandidType, Principal};
use serde::Deserialize;

// proxy principal
const PROXY_PRINCIPAL: &str = "bwm3m-wyaaa-aaaag-qdiua-cai";

// proxy logs
#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct Logger {
    pub description: String,
    pub source: Option<String>,
    pub principal: Option<Principal>,
    pub data: Option<String>,
    pub created_on: Timestamp,
}

pub async fn get_latest_proxy_logs(amount: u64) -> Vec<Logger> {
    ic_cdk::call::<(u64,), (Vec<Logger>,)>(
        Principal::from_text(PROXY_PRINCIPAL).expect("Invalid principal"),
        "get_latest_logs",
        (amount,),
    )
    .await
    .expect("Failed to call get_latest_logs")
    .0
}

pub async fn log_size() -> u64 {
    ic_cdk::call::<(), (u64,)>(
        Principal::from_text(PROXY_PRINCIPAL).expect("Invalid principal"),
        "log_size",
        (),
    )
    .await
    .expect("Failed to call log_size")
    .0
}

// proxy reward buffer
#[derive(CandidType, Deserialize, Clone)]
pub struct RewardableActivity {
    pub timestamp: u64,
    pub activity: Vec<u8>,
}

pub async fn read_reward_buffer() -> Vec<RewardableActivity> {
    ic_cdk::call::<(), (Vec<RewardableActivity>,)>(
        Principal::from_text(PROXY_PRINCIPAL).expect("Invalid principal"),
        "read_reward_buffer",
        (),
    )
    .await
    .expect("Failed to call read_reward_buffer")
    .0
}

pub async fn reward_timer_next_trigger() -> Option<u64> {
    ic_cdk::call::<(), (Option<u64>,)>(
        Principal::from_text(PROXY_PRINCIPAL).expect("Invalid principal"),
        "reward_timer_next_trigger",
        (),
    )
    .await
    .expect("Failed to call reward_timer_next_trigger")
    .0
}

// proxy stats
pub async fn proxy_store_stats() -> Vec<String> {
    ic_cdk::call::<(), (Vec<String>,)>(
        Principal::from_text(PROXY_PRINCIPAL).expect("Invalid principal"),
        "store_stats",
        (),
    )
    .await
    .expect("Failed to call store_stats")
    .0
}
