use candid::{CandidType, Principal};
use serde::Deserialize;

const TOKEN_CANISTER_PRINCIPAL: &str = "zgfl7-pqaaa-aaaap-accpa-cai";

#[derive(Deserialize, CandidType, Clone)]
pub struct RewardData {
    pub timestamp: u64,
    pub principal: Principal,
    pub description: String,
}

pub async fn latest_rewards(amount: u64) -> Vec<RewardData> {
    ic_cdk::call::<(u64,), (Vec<RewardData>,)>(
        Principal::from_text(TOKEN_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "latest_rewards",
        (amount,),
    )
    .await
    .expect("Failed to call get_latest_logs")
    .0
}

pub async fn token_balances() -> Vec<(Principal, u64)> {
    ic_cdk::call::<(), (Vec<(Principal, u64)>,)>(
        Principal::from_text(TOKEN_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "token_balances",
        (),
    )
    .await
    .expect("Failed to call token_balances")
    .0
}

pub async fn log_size() -> u64 {
    ic_cdk::call::<(), (u64,)>(
        Principal::from_text(TOKEN_CANISTER_PRINCIPAL).expect("Invalid principal"),
        "log_size",
        (),
    )
    .await
    .expect("Failed to call log_size")
    .0
}
