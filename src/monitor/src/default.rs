use crate::store::STATE;
use ic_cdk::api::time;
use ic_cdk::{call, id};
use ic_cdk_macros::{init, update};
use ic_cdk_timers::set_timer_interval;
use ic_ledger_types::Tokens;
use std::time::Duration;

// const INTERVAL: u64 = 24 * 60 * 60 * 1_000_000_000; // 1 day
const INTERVAL: Duration = Duration::from_secs(10);

#[init]
fn init() {
    let timer_id = set_timer_interval(INTERVAL, move || ic_cdk::spawn(operations()));

    STATE.with(|s| s.borrow_mut().set_timer_id(timer_id));
}

async fn operations() {
    let now = time();
    let balance = call::<(), (Tokens,)>(id(), "get_icp_balance", ())
        .await
        .expect("Failed to query ledger canister for balance")
        .0;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.set_last_poll_time(now);
        state.set_icp_balance(balance);
    });
}
