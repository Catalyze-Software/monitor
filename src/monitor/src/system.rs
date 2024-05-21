use crate::run::run;
use ic_cdk_macros::{init, post_upgrade};
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(3 * 60 * 60); // 3 hours

// thread local refcell for timer id
thread_local! {
   pub static TIMER: std::cell::RefCell<Option<ic_cdk_timers::TimerId>> = std::cell::RefCell::new(None);
}

#[init]
fn init() {
    // set timer to run operations at INTERVAL
    // first run will be INTERVAL after the canister is upgraded/ reinstalled
    // call `initiate_run` to start the first run immediately
    let id = set_timer_interval(INTERVAL, move || ic_cdk::spawn(run()));

    TIMER.with(|t| *t.borrow_mut() = Some(id));
}

#[post_upgrade]
fn post_upgrade() {
    init();
}
