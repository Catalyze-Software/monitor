use crate::stores::stable_store::MonitorStore;
use crate::utils::auth::is_authenticated;
use crate::utils::sort::cycle_balances;
use crate::{run::run, stores::stable_store::Logs};
use ic_cdk_macros::{query, update};

#[query(guard = "is_authenticated")]
fn icp_balance() -> String {
    let monitor_state = MonitorStore::get_latest().expect("No monitor state");
    format!("{}", monitor_state.icp_balance)
}

#[query(guard = "is_authenticated")]
fn latest_icp_balances(n: u64) -> Vec<String> {
    MonitorStore::get_latest_icp_balances(n)
}

#[query(guard = "is_authenticated")]
fn all_cycle_balances() -> Vec<String> {
    cycle_balances()
}

#[update(guard = "is_authenticated")]
async fn update_state() {
    ic_cdk::spawn(run())
}

#[query(guard = "is_authenticated")]
fn get_log(n: u64) -> Vec<String> {
    Logs::get_latest(n)
}

#[query(guard = "is_authenticated")]
fn get_latest_with_timestamp(n: u64) -> Vec<String> {
    Logs::get_latest_with_timestamps(n)
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
