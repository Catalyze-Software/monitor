use crate::queries::cycle_history::{get_latest_cycle_balances, CycleBalances};
use crate::queries::icp_history::get_latest_icp_balances;
use crate::queries::sort::cycle_balances;
use crate::stores::stable_models::{CanisterCycles, Timestamp};
use crate::stores::stable_store::{ChildStore, MonitorStore, SnsStore};
use crate::utils::auth::is_authenticated;
use crate::{run::run, stores::stable_store::Logs};
use ic_cdk_macros::{query, update};

#[query(guard = "is_authenticated")]
fn icp_balance() -> String {
    let monitor_state = MonitorStore::get_latest().expect("No monitor state");
    format!("{}", monitor_state.icp_balance)
}

#[query(guard = "is_authenticated")]
fn latest_icp_balances(n: u64) -> Vec<(Timestamp, f64)> {
    get_latest_icp_balances(n)
}

#[query(guard = "is_authenticated")]
fn latest_cycle_balances(n: u64) -> Vec<CycleBalances> {
    get_latest_cycle_balances(n)
}

#[query(guard = "is_authenticated")]
fn all_cycle_balances() -> Vec<String> {
    cycle_balances()
}

#[query(guard = "is_authenticated")]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::queries::sort::sorted_canister_cycles()
}

#[update(guard = "is_authenticated")]
async fn initiate_run() {
    ic_cdk::spawn(run())
}

#[query(guard = "is_authenticated")]
fn get_latest_with_timestamp(n: u64) -> Vec<String> {
    Logs::get_latest_with_timestamps(n)
}

#[query(guard = "is_authenticated")]
fn store_stats() -> Vec<String> {
    vec![
        format!("Logs: {}", Logs::size()),
        format!("Monitor: {}", MonitorStore::size()),
        format!("SNS: {}", SnsStore::size()),
        format!("Child: {}", ChildStore::size()),
        format!("Logs index: {}", Logs::new_index()),
        format!("Monitor index: {}", MonitorStore::new_index()),
        format!("SNS index: {}", SnsStore::new_index()),
        format!("Child index: {}", ChildStore::new_index()),
    ]
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
