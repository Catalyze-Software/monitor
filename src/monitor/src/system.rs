use crate::{run::run, stores::store::TIMER};
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

#[pre_upgrade]
fn pre_upgrade() {
    TIMER.with(|t| {
        let timer = t.borrow();
        clear_timer(timer.get_timer_id());
    });
}

#[post_upgrade]
fn post_upgrade() {
    init();
}
