use crate::{
    log::{log, EVENT_CHILD_SUMMARY, EVENT_CYCLE_BALANCE, EVENT_ICP_BALANCE, EVENT_SNS_SUMMARY},
    store::STATE,
};
use ic_cdk::api::time;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use ic_cdk_timers::{clear_timer, set_timer, set_timer_interval};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

/*
* One global timer is set at each canister upgrade / reinstall.
* The timer invokes an async fn `operations` from the `init` sync context
* Timer is cleared at each canister upgrade / reinstall and set again in `post_upgrade`
* Child ops are seperated because they currently trap (monitor is not controller)
*/

#[init]
fn init() {
    // run operations once immediately to init state
    // need to use timer to spawn async fn from sync context
    let _ = set_timer(Duration::from_nanos(1), move || {
        ic_cdk::spawn(operations());
        ic_cdk::spawn(child_operations());
    });

    // set timer to run operations at INTERVAL
    let timer_id = set_timer_interval(INTERVAL, move || {
        ic_cdk::spawn(operations());
        ic_cdk::spawn(child_operations());
    });

    STATE.with(|s| s.borrow_mut().set_timer_id(timer_id));
}

/*
* Perform SNS canisters query routine and top-up if needed
*/
pub async fn operations() {
    let now = time();
    let balance = crate::operations::ledger::icp_balance().await;
    let cycles = crate::operations::cmc::cycle_balance().await;
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();

        state.set_last_poll_time(now);
        state.set_icp_balance(balance);
        state.set_cycle_balance(cycles.clone());
        state.set_summary(summary);
    });

    // log the wuery operations
    log(format!("{}: {}", EVENT_ICP_BALANCE.to_string(), balance));
    log(format!("{}: {}", EVENT_CYCLE_BALANCE.to_string(), cycles));
    log(EVENT_SNS_SUMMARY.to_string());

    // top up canisters with low cycles after state update
    crate::operations::charge::top_up_sns_canisters().await;
}

pub async fn child_operations() {
    let childs = crate::operations::child::get_child_canister_summary().await;
    STATE.with(|s| {
        s.borrow_mut().set_childs(childs);
    });

    log(EVENT_CHILD_SUMMARY.to_string());

    // top up child canisters with low cycles after state update
    crate::operations::charge::top_up_child_canisters().await;
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
