use crate::{
    log::format_time,
    operations::sns::{CanisterSummary, GetSnsCanistersSummaryResponse},
};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use ic_cdk_timers::TimerId;
use ic_ledger_types::Tokens;
use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
    pub static TIMER: RefCell<Timer> = RefCell::new(Timer::default());
}

#[derive(Default, Clone, CandidType, Deserialize)]
pub struct State {
    log: Vec<(u64, String)>,
    last_poll_time: Option<u64>,

    icp_balance: Option<Tokens>,
    cycle_balance: Option<Nat>,

    summary: Option<GetSnsCanistersSummaryResponse>,
    childs: Option<Vec<CanisterCycles>>,
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

    pub fn set_childs(&mut self, childs: Vec<CanisterCycles>) {
        self.childs = Some(childs);
    }

    pub fn get_childs(&self) -> Option<Vec<CanisterCycles>> {
        self.childs.clone()
    }
}

#[derive(Default)]
pub struct Timer {
    timer_id: Option<TimerId>,
}

impl Timer {
    pub fn set_timer_id(&mut self, timer_id: TimerId) {
        self.timer_id = Some(timer_id);
    }

    pub fn get_timer_id(&self) -> TimerId {
        self.timer_id.expect("Timer id not set")
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterCycles {
    pub name: String,
    pub canister_id: Principal,
    pub cycles: Nat,
}

impl CanisterCycles {
    pub fn new<'a>(name: &str, canister_summary: &CanisterSummary) -> Self {
        Self {
            name: String::from(name),
            canister_id: canister_summary.canister_id.unwrap(),
            cycles: canister_summary.status.as_ref().unwrap().cycles.clone(),
        }
    }

    pub fn from_status<'a>(name: &str, canister_id: &str, status: &CanisterStatusResponse) -> Self {
        Self {
            name: String::from(name),
            canister_id: Principal::from_text(canister_id).expect("Invalid principal"),
            cycles: Nat::from(status.cycles.clone()),
        }
    }
}
