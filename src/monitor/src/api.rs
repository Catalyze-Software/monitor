use crate::stores::stable_store::MonitorStore;
use crate::utils::auth::is_authenticated;
use crate::{
    run::run,
    stores::{stable_models::CanisterCycles, stable_store::Logs},
};
use ic_cdk_macros::{query, update};

#[query(guard = "is_authenticated")]
fn icp_balance() -> String {
    let monitor_state = MonitorStore::get_latest().expect("No monitor state");
    format!("{}", monitor_state.icp_balance)
}

#[query(guard = "is_authenticated")]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::utils::sort::sorted_canister_cycles()
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
