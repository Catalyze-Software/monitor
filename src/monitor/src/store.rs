use crate::{sns::GetSnsCanistersSummaryResponse, utils::format_time};
use candid::Nat;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use ic_cdk::api::time;
use ic_cdk_timers::TimerId;
use ic_ledger_types::Tokens;
use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    log: Vec<(u64, String)>,

    timer_id: Option<TimerId>,
    last_poll_time: Option<u64>,

    icp_balance: Option<Tokens>,
    cycle_balance: Option<Nat>,

    summary: Option<GetSnsCanistersSummaryResponse>,
    pub childs: Option<Vec<(String, CanisterStatusResponse)>>,
}

impl State {
    pub fn log(&mut self, timestamp: u64, msg: String) {
        self.log.push((timestamp, msg));
    }

    pub fn get_log(&self, n: usize) -> Vec<String> {
        self.log
            .iter()
            .rev()
            .take(n)
            .map(|(timestamp, msg)| format!("{} {}", format_time(*timestamp), msg.clone()))
            .collect()
    }

    pub fn set_timer_id(&mut self, timer_id: TimerId) {
        self.timer_id = Some(timer_id);
    }

    pub fn get_timer_id(&self) -> TimerId {
        self.timer_id.expect("TimerId not set")
    }

    pub fn set_last_poll_time(&mut self, poll_time: u64) {
        self.last_poll_time = Some(poll_time);
    }

    pub fn get_last_poll_time(&self) -> u64 {
        self.last_poll_time.expect("Last poll time not set")
    }

    pub fn set_icp_balance(&mut self, balance: Tokens) {
        self.icp_balance = Some(balance);
    }

    pub fn get_icp_balance(&self) -> Tokens {
        self.icp_balance.expect("ICP balance not set")
    }

    pub fn set_cycle_balance(&mut self, balance: Nat) {
        self.cycle_balance = Some(balance);
    }

    pub fn get_cycle_balance(&self) -> Nat {
        self.cycle_balance.clone().expect("Cycle balance not set")
    }

    pub fn set_summary(&mut self, summary: GetSnsCanistersSummaryResponse) {
        self.summary = Some(summary);
    }

    pub fn get_summary(&self) -> GetSnsCanistersSummaryResponse {
        self.summary.clone().expect("Summary not set")
    }

    pub fn set_childs(&mut self, childs: Vec<(String, CanisterStatusResponse)>) {
        self.childs = Some(childs);
    }
}

pub fn log(msg: String) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.log(time(), msg);
    });
}
