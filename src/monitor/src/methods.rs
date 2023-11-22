use crate::{sns::CanisterCycles, store::STATE};
use ic_cdk_macros::query;

#[query]
fn last_poll_time() -> u64 {
    STATE.with(|s| s.borrow().get_last_poll_time())
}

#[query]
fn icp_balance() -> u64 {
    STATE.with(|s| s.borrow().get_icp_balance().e8s())
}

#[query]
fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    crate::sns::sorted_canister_cycles()
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
