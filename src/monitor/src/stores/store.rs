use ic_cdk_timers::TimerId;
use std::cell::RefCell;

thread_local! {
    pub static TIMER: RefCell<Timer> = RefCell::new(Timer::default());
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
