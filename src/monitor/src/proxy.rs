use candid::{CandidType, Principal};
use serde::Deserialize;

// proxy principal
const PROXY_PRINCIPAL: &str = "bwm3m-wyaaa-aaaag-qdiua-cai";

pub type Timestamp = u64;

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
