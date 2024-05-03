use ic_cdk::trap;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

pub const EVENT_MONITOR_DATA: &str = "Fetched and stored monitor data";
pub const EVENT_SNS_DATA: &str = "Fetched and stored SNS canisters summary";
pub const EVENT_CATALYZE_CANISTER_DATA: &str = "Fetched and stored Catalyze canisters";
pub const EVENT_COMPLETED_READ_OPERATION: &str = "Completed full snapshot";

pub const EVENT_ICP_SENT: &str = "ICP sent";
pub const EVENT_CYCLES_MINTED: &str = "Cycles minted";

pub fn format_time(timestamp: u64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128)
        .unwrap_or_else(|e| trap(&format!("Error converting timestamp: {}", e)));

    datetime
        .format(&Rfc2822)
        .unwrap_or_else(|e| trap(&format!("Error formatting datetime: {}", e)))
}
