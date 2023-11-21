use crate::{
    sns::{get_sns_canisters_summary, GetSnsCanistersSummaryResponse},
    store::STATE,
};
use ic_cdk_macros::update;

#[update]
async fn get_sns_canisters_summary_test() -> GetSnsCanistersSummaryResponse {
    get_sns_canisters_summary().await;

    STATE.with(|s| s.borrow().summary.clone())
}

#[test]
fn generate_candid() {
    candid::export_service!();

    ic_scalable_misc::helpers::candid_helper::save_candid(
        __export_service(),
        String::from("monitor"),
    );
}
