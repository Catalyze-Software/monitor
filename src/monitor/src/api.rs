use crate::queries::cycle_history::{get_latest_cycle_balances, CycleBalances};
use crate::queries::icp_history::get_latest_icp_balances;
use crate::queries::sort::cycle_balances;
use crate::stores::stable_models::{CanisterCycles, Log, Timestamp};
use crate::stores::stable_store::{ChildStore, FrontendStore, MonitorStore, SnsStore};
use crate::utils::auth::is_registered;
use crate::{run::run, stores::stable_store::Logs};
use candid::Principal;
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
fn all_cycle_balances() -> Vec<String> {
    cycle_balances()
}

#[query(guard = "is_registered")]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::queries::sort::sorted_canister_cycles()
}

#[update(guard = "is_registered")]
async fn initiate_run() {
    ic_cdk::spawn(run())
}

#[query(guard = "is_registered")]
fn get_latest_logs(n: u64) -> Vec<Log> {
    Logs::get_latest(n)
}

#[query(guard = "is_registered")]
fn get_latest_with_timestamp(n: u64) -> Vec<String> {
    Logs::get_latest_with_timestamps(n)
}

#[query(guard = "is_registered")]
fn store_stats() -> Vec<String> {
    vec![
        format!("Logs: {}", Logs::size()),
        format!("Monitor: {}", MonitorStore::size()),
        format!("SNS: {}", SnsStore::size()),
        format!("Child: {}", ChildStore::size()),
        format!("Frontend: {}", FrontendStore::size()),
        format!("Logs index: {}", Logs::new_index()),
        format!("Monitor index: {}", MonitorStore::new_index()),
        format!("SNS index: {}", SnsStore::new_index()),
        format!("Child index: {}", ChildStore::new_index()),
        format!("Frontend index: {}", FrontendStore::new_index()),
    ]
}

#[query]
fn new_user() -> Option<Principal> {
    match is_registered() {
        Ok(_) => None,
        _ => Some(ic_cdk::caller()),
    }
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
