use ic_cdk_timers::TimerId;
use ic_ledger_types::Tokens;

use crate::sns::GetSnsCanistersSummaryResponse;
use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    timer_id: Option<TimerId>,
    last_poll_time: Option<u64>,
    icp_balance: Option<Tokens>,
    summary: Option<GetSnsCanistersSummaryResponse>,
}

impl State {
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

    pub fn set_summary(&mut self, summary: GetSnsCanistersSummaryResponse) {
        self.summary = Some(summary);
    }

    pub fn get_summary(&self) -> GetSnsCanistersSummaryResponse {
        self.summary.clone().expect("Summary not set")
    }
}
