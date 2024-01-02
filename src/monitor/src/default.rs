use crate::{
    log::{log, EVENT_CHILD_SUMMARY, EVENT_CYCLE_BALANCE, EVENT_ICP_BALANCE, EVENT_SNS_SUMMARY},
    operations::charge::{top_up_child_canisters, top_up_sns_canisters},
    store::{State, STATE, TIMER},
};
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_cdk::storage::{stable_restore, stable_save};
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
    let _ = set_timer(Duration::from_nanos(1), move || ic_cdk::spawn(run()));

    // set timer to run operations at INTERVAL
    let timer_id = set_timer_interval(INTERVAL, move || ic_cdk::spawn(run()));

    TIMER.with(|t| t.borrow_mut().set_timer_id(timer_id));
}

/*
* Perform a full run of state update and charge operations
*/
pub async fn run() {
    // read SNS canister summary
    operations().await;
    // top up sns canisters if needed
    top_up_sns_canisters().await;
    // read SNS canister summary again after top-up
    operations().await;

    // // read child canister summary
    // ic_cdk::spawn(child_operations());
    // // top up child canisters if needed
    // ic_cdk::spawn(top_up_child_canisters());
    // // read child canister summary again after top-up
    // ic_cdk::spawn(child_operations());
}

/*
* Perform SNS canisters query routine and top-up if needed
*/
async fn operations() {
    let balance = crate::operations::ledger::icp_balance().await;
    let cycles = crate::operations::cmc::cycle_balance().await;
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();

        state.set_last_poll_time(time());
        state.set_icp_balance(balance);
        state.set_cycle_balance(cycles.clone());
        state.set_summary(summary);
    });

    // log the query operations
    log(format!("{}: {}", EVENT_ICP_BALANCE.to_string(), balance));
    log(format!("{}: {}", EVENT_CYCLE_BALANCE.to_string(), cycles));
    log(EVENT_SNS_SUMMARY.to_string());
}

pub async fn child_operations() {
    let childs = crate::operations::child::get_child_canister_summary().await;
    STATE.with(|s| {
        s.borrow_mut().set_childs(childs);
    });

    log(EVENT_CHILD_SUMMARY.to_string());

    // top up child canisters with low cycles after state update
    top_up_child_canisters().await;
}

#[pre_upgrade]
fn pre_upgrade() {
    TIMER.with(|t| {
        let timer = t.borrow();
        clear_timer(timer.get_timer_id());
    });

    let serialized = Encode!(&STATE.with(|s| s.borrow().clone())).unwrap();
    stable_save::<(Vec<u8>,)>((serialized,)).expect("Failed to save state");
}

#[post_upgrade]
fn post_upgrade() {
    let (serialized,) = stable_restore::<(Vec<u8>,)>().expect("Failed to restore state");
    let state = candid::Decode!(&serialized, State).unwrap();

    STATE.with(|s| {
        *s.borrow_mut() = state;
    });
    init();
}
