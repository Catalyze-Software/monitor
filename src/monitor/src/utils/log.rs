use ic_cdk::trap;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

pub const EVENT_MONITOR_DATA: &str = "Fetched and stored monitor data";
pub const EVENT_SNS_DATA: &str = "Fetched and stored SNS canisters summary";
pub const EVENT_CHILD_SUMMARY: &str = "Fetched and stored child canisters summary";
pub const EVENT_ICP_SENT: &str = "ICP sent";
pub const EVENT_CYCLES_MINTED: &str = "Cycles minted";
pub const EVENT_FRONTEND_SUMMARY: &str = "Fetched and stored frontend canister summary";
pub const EVENT_SIWE_SUMMARY: &str = "Fetched and stored Siwe canister summary";
pub const EVENT_SIWS_SUMMARY: &str = "Fetched and stored Siws canister summary";
pub const EVENT_DASHBOARD_SUMMARY: &str = "Fetched and stored dashboard summary";

pub fn format_time(timestamp: u64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128)
        .unwrap_or_else(|e| trap(&format!("Error converting timestamp: {}", e)));

    datetime
        .format(&Rfc2822)
        .unwrap_or_else(|e| trap(&format!("Error formatting datetime: {}", e)))
}
