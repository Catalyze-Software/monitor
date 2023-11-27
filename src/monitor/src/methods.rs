use crate::{
    default::{child_operations, operations},
    log::format_time,
    sort::CanisterCycles,
    store::STATE,
};
use candid::Nat;
use ic_cdk_macros::{query, update};

#[query]
fn last_poll_time() -> String {
    format_time(STATE.with(|s| s.borrow().get_last_poll_time()))
}

#[query]
fn icp_balance() -> u64 {
    STATE.with(|s| s.borrow().get_icp_balance().e8s())
}

#[query]
fn cycle_balance() -> Nat {
    STATE.with(|s| s.borrow().get_cycle_balance())
}

#[query]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::sort::sorted_canister_cycles()
}

#[update]
fn update_state() {
    ic_cdk::spawn(operations());
    ic_cdk::spawn(child_operations());
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
