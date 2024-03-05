use crate::run::run;
use ic_cdk_macros::{init, post_upgrade};
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

#[init]
fn init() {
    // set timer to run operations at INTERVAL
    // first run will be INTERVAL after the canister is upgraded/ reinstalled
    // call `initiate_run` to start the first run immediately
    let _ = set_timer_interval(INTERVAL, move || ic_cdk::spawn(run()));
}

#[post_upgrade]
fn post_upgrade() {
    init();
}
