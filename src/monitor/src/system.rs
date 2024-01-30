use crate::run::run;
use ic_cdk_macros::{init, post_upgrade};
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

#[init]
fn init() {
    // run operations once immediately to init state
    // need to use timer to spawn async fn from sync context
    // let _ = set_timer(Duration::from_nanos(1), move || ic_cdk::spawn(run()));

    // set timer to run operations at INTERVAL
    let _ = set_timer_interval(INTERVAL, move || ic_cdk::spawn(run()));
}

#[post_upgrade]
fn post_upgrade() {
    init();
}
