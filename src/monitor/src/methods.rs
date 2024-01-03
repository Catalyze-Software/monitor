use crate::{
    default::run,
    log::format_time,
    store::{CanisterCycles, STATE},
};
use ic_cdk_macros::{query, update};
use ic_ledger_types::Tokens;

#[query]
fn last_poll_time() -> (String, u64) {
    let time = STATE.with(|s| s.borrow().get_last_poll_time());
    (format_time(time), time)
}

#[query]
fn icp_balance() -> (String, u64) {
    let balance = STATE.with(|s| s.borrow().get_icp_balance());
    (format!("{}", balance), Tokens::e8s(&balance))
}

#[query]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::sort::sorted_canister_cycles()
}

#[update]
async fn update_state() {
    ic_cdk::spawn(run())
}

#[query]
fn get_log(n: usize) -> Vec<String> {
    STATE.with(|s| s.borrow().get_log(n))
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
