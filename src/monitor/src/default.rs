use crate::store::STATE;
use ic_cdk::api::time;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use ic_cdk_timers::{clear_timer, set_timer_interval};
use std::time::Duration;

// const INTERVAL: u64 = 24 * 60 * 60 * 1_000_000_000; // 1 day
const INTERVAL: Duration = Duration::from_secs(10);

/*
* One global timer is set at each canister upgrade / reinstall.
* The timer invokes an async fn `operations` from the `init` sync context
* Timer is cleared at each canister upgrade / reinstall and set again in `post_upgrade`
*/

#[init]
fn init() {
    let timer_id = set_timer_interval(INTERVAL, move || ic_cdk::spawn(operations()));

    STATE.with(|s| s.borrow_mut().set_timer_id(timer_id));
}

async fn operations() {
    let now = time();
    let balance = crate::ledger::icp_balance().await;
    let summary = crate::sns::get_sns_canisters_summary().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();

        state.set_last_poll_time(now);
        state.set_icp_balance(balance);
        state.set_summary(summary);
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        let state = s.borrow();

        clear_timer(state.get_timer_id());
    });
}

#[post_upgrade]
fn post_upgrade() {
    init();
}
