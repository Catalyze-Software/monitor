use crate::canisters::proxy::Logger;
use crate::queries::{get_latest_cycle_balances, get_latest_icp_balances};
use crate::stores::stable_store::{CanisterStatusStore, MonitorStore};
use crate::stores::types::{CanisterCycles, CanisterMemorySize, CycleBalances, Log, Timestamp};
use crate::system::TIMER;
use crate::token_canister::RewardData;
use crate::utils::auth::is_registered;
use crate::{run::run, stores::stable_store::Logs};
use ic_cdk_macros::{query, update};

#[query(guard = "is_registered")]
fn icp_balance() -> String {
    let monitor_state = MonitorStore::get_latest().expect("No monitor state");
    format!("{}", monitor_state.icp_balance)
}

#[query(guard = "is_registered")]
fn latest_icp_balances(n: u64) -> Vec<(Timestamp, f64)> {
    get_latest_icp_balances(n)
}

#[query(guard = "is_registered")]
fn latest_cycle_balances(n: u64) -> Vec<CycleBalances> {
    get_latest_cycle_balances(n)
}

#[query(guard = "is_registered")]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let latest_snapshot = CanisterStatusStore::get_latest().expect("No latest snapshot");
    crate::queries::sorted_canister_cycles(&latest_snapshot)
}

#[query(guard = "is_registered")]
fn sorted_memory_sizes() -> Vec<CanisterMemorySize> {
    let latest_snapshot = CanisterStatusStore::get_latest().expect("No latest snapshot");
    crate::queries::sorted_memory_sizes(&latest_snapshot)
}

#[update(guard = "is_registered")]
async fn initiate_run() {
    ic_cdk::spawn(run())
}

#[query(guard = "is_registered")]
fn get_latest_logs(n: u64) -> Vec<Log> {
    Logs::get_latest(n)
}

#[update(guard = "is_registered")]
async fn latest_proxy_logs(amount: u64) -> Vec<Logger> {
    crate::canisters::proxy::get_latest_proxy_logs(amount).await
}

#[update(guard = "is_registered")]
async fn proxy_log_size() -> u64 {
    crate::canisters::proxy::log_size().await
}

#[update(guard = "is_registered")]
async fn token_latest_rewards(amount: u64) -> Vec<RewardData> {
    crate::token_canister::latest_rewards(amount).await
}

#[update(guard = "is_registered")]
async fn token_log_size() -> u64 {
    crate::token_canister::log_size().await
}

#[query(guard = "is_registered")]
fn get_latest_with_timestamp(n: u64) -> Vec<String> {
    Logs::get_latest_with_timestamps(n)
}

#[query(guard = "is_registered")]
fn store_stats() -> Vec<String> {
    vec![
        // Stores sizes
        format!("Logs: {}", Logs::size()),
        format!("Monitor: {}", MonitorStore::size()),
        format!("CanisterStatusStore: {}", CanisterStatusStore::size()),
        // Indexes
        format!("Logs index: {}", Logs::new_index()),
        format!("Monitor index: {}", MonitorStore::new_index()),
        format!(
            "CanisterStatusStore index: {}",
            CanisterStatusStore::new_index()
        ),
    ]
}

#[query(guard = "is_registered")]
fn timer_set() -> bool {
    match TIMER.with(|t| t.borrow().clone()) {
        Some(_) => true,
        None => false,
    }
}

#[test]
fn generate_candid() {
    candid::export_service!();

    std::fs::write("../../candid/monitor.did", __export_service())
        .expect("Unable to write did file");
}
