use time::format_description::well_known::Iso8601;

use crate::{sns::CanisterCycles, store::STATE};
use ic_cdk::trap;
use ic_cdk_macros::query;
use time::OffsetDateTime;

#[query]
fn last_poll_time() -> String {
    let poll_time = STATE.with(|s| s.borrow().get_last_poll_time());

    let datetime = OffsetDateTime::from_unix_timestamp(poll_time as i64)
        .unwrap_or_else(|e| trap(&format!("Error converting timestamp: {}", e)));

    datetime
        .format(&Iso8601::DEFAULT)
        .unwrap_or_else(|e| trap(&format!("Error formatting datetime: {}", e)))
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
