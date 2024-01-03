use crate::{
    log::{log, EVENT_CHILD_SUMMARY, EVENT_CYCLE_BALANCE, EVENT_ICP_BALANCE, EVENT_SNS_SUMMARY, EVENT_READ_OPERATIONS, EVENT_TOP_UP_CANISTERS},
    operations::charge::top_up_canisters,
    store::{State, STATE, TIMER},
};
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use ic_cdk_timers::{clear_timer, set_timer, set_timer_interval};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

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
    read_operations().await;
    log(EVENT_READ_OPERATIONS.to_string());

    top_up_canisters().await;
    log(EVENT_TOP_UP_CANISTERS.to_string());

    read_operations().await;
    log(EVENT_READ_OPERATIONS.to_string());

    log("RUN COMPLETED SUCCESFULLY".to_string());
}

/*
* Perform SNS canisters query routine and top-up if needed
*/
async fn read_operations() {
    // Monitor ICP balance
    let balance = crate::operations::ledger::icp_balance().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.set_icp_balance(balance);
    });

    log(format!("{}: {}", EVENT_ICP_BALANCE.to_string(), balance));

    // Monitor cycles balance
    let cycles = crate::operations::cmc::cycle_balance().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.set_cycle_balance(cycles.clone());
    });

    log(format!("{}: {}", EVENT_CYCLE_BALANCE.to_string(), cycles));

    // SNS canisters summary
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.set_summary(summary);
    });

    log(EVENT_SNS_SUMMARY.to_string());

    // Child canisters summary
    let childs = crate::operations::child::get_child_canister_summary().await;

    STATE.with(|s| {
        s.borrow_mut().set_childs(childs);
    });

    log(EVENT_CHILD_SUMMARY.to_string());

    // Set last poll time
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.set_last_poll_time(time());
    });
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
