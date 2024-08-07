use crate::canisters::proxy::{Logger, RewardableActivity};
use crate::canisters::rewards::GroupInfo;
use crate::queries::get_latest_icp_balances;
use crate::stores::stable_store::{CanisterStatusStore, MonitorStore};
use crate::stores::types::{CanisterCycles, CanisterMemorySize, CycleHistory, Log, Timestamp};
use crate::system::TIMER;
use crate::utils::auth::is_registered;
use crate::{run::run, stores::stable_store::Logs};
use candid::Principal;
use ic_cdk_macros::{query, update};

// Monitor interface
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
fn canister_cycle_history(amount: u64) -> CycleHistory {
    crate::queries::canister_cycle_history(amount)
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
    run().await;
}

#[query(guard = "is_registered")]
fn get_latest_logs(n: u64) -> Vec<Log> {
    Logs::get_latest(n)
}

#[query(guard = "is_registered")]
fn get_latest_with_timestamp(n: u64) -> Vec<String> {
    Logs::get_latest_with_timestamps(n)
}

// Proxy interface
#[update(guard = "is_registered")]
async fn latest_proxy_logs(amount: u64) -> Vec<Logger> {
    crate::canisters::proxy::get_latest_proxy_logs(amount).await
}

#[update(guard = "is_registered")]
async fn proxy_log_size() -> u64 {
    crate::canisters::proxy::log_size().await
}

#[update(guard = "is_registered")]
async fn read_reward_buffer() -> Vec<RewardableActivity> {
    crate::canisters::proxy::read_reward_buffer().await
}

#[update(guard = "is_registered")]
async fn reward_timer_next_trigger() -> Option<u64> {
    crate::canisters::proxy::reward_timer_next_trigger().await
}

#[update(guard = "is_registered")]
async fn proxy_store_stats() -> Vec<String> {
    crate::canisters::proxy::proxy_store_stats().await
}

// Rewards interface
#[update(guard = "is_registered")]
async fn group_info() -> Vec<GroupInfo> {
    crate::canisters::rewards::group_info().await
}

// #[update(guard = "is_registered")]
// async fn event_info() -> Vec<EventInfo> {
//     crate::canisters::rewards::event_info().await
// }

#[update(guard = "is_registered")]
async fn token_balances() -> Vec<(Principal, u64)> {
    crate::canisters::rewards::token_balances().await
}

#[update(guard = "is_registered")]
async fn token_log_size() -> u64 {
    crate::canisters::rewards::log_size().await
}

#[update(guard = "is_registered")]
async fn graph_member_count_rewards() -> Vec<(u64, u64)> {
    crate::canisters::rewards::graph_member_count_rewards().await
}

#[update(guard = "is_registered")]
async fn graph_member_activity_rewards() -> Vec<(u64, u64)> {
    crate::canisters::rewards::graph_member_activity_rewards().await
}

#[update(guard = "is_registered")]
async fn graph_event_attendee_rewards() -> Vec<(u64, u64)> {
    crate::canisters::rewards::graph_event_attendee_rewards().await
}

#[update]
fn test_log(log: String) {
    Logs::log(log)
}

// Monitor system interface
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
    TIMER.with(|t| *t.borrow()).is_some()
}

#[test]
fn generate_candid() {
    candid::export_service!();

    std::fs::write("../../candid/monitor.did", __export_service())
        .expect("Unable to write did file");
}
