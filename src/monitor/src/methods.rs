use crate::store::STATE;
use ic_cdk_macros::{query, update};
use ic_ledger_types::Tokens;

// #[update]
// async fn get_sns_canisters_summary_test() -> GetSnsCanistersSummaryResponse {
//     get_sns_canisters_summary().await;

//     STATE.with(|s| s.borrow().summary.clone())
// }

#[query]
fn last_poll_time() -> u64 {
    STATE.with(|s| s.borrow().get_last_poll_time())
}

#[query]
fn icp_balance() -> u64 {
    STATE.with(|s| s.borrow().get_icp_balance().e8s())
}

#[update]
async fn get_icp_balance() -> Tokens {
    crate::ledger::icp_balance().await
}

#[test]
fn generate_candid() {
    use ic_ledger_types::Tokens;
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
