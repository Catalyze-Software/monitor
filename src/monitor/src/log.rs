use crate::store::STATE;
use ic_cdk::api::time;
use ic_cdk::trap;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

pub const EVENT_ICP_BALANCE: &str = "Updated ICP balance";
pub const EVENT_CYCLE_BALANCE: &str = "Updated cycle balance";

pub const EVENT_SNS_SUMMARY: &str = "Updated SNS summary";
pub const EVENT_CHILD_SUMMARY: &str = "Updated child summary";

pub const EVENT_ICP_SENT: &str = "ICP sent";
pub const EVENT_CYCLES_MINTED: &str = "Cycles minted";

pub const EVENT_READ_OPERATIONS: &str = "Read operations successful";
pub const EVENT_TOP_UP_CANISTERS: &str = "Top up canisters successful";

pub fn log(msg: String) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.log(time(), msg);
    });
}

pub fn format_time(timestamp: u64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128)
        .unwrap_or_else(|e| trap(&format!("Error converting timestamp: {}", e)));

    datetime
        .format(&Rfc2822)
        .unwrap_or_else(|e| trap(&format!("Error formatting datetime: {}", e)))
}
