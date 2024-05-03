mod api;
mod operations;
mod proxy;
mod queries;
mod run;
mod stores;
mod system;
mod token_canister;
mod utils;

/*
 * Canister names and IDs need to be in same order
 * This list contains all canisters that need to be fetched manually
 * This list excludes SNS canisters which are fetched at once with `get_sns_canisters_summary`
 */

// Canister names
pub const CANISTER_NAMES: [&str; 11] = [
    // monitor and dashboard
    "Monitor",
    "Dashboard",
    // catalyze canisters
    "Frontend",
    "SIWE",
    "SIWS",
    // child production names
    "Members",
    "Groups",
    "Profiles",
    "Events",
    "Event attendees",
    "Reports",
];
// Canister list
pub const CANISTER_IDS: [&str; 11] = [
    // monitor and dashboard
    "6or45-oyaaa-aaaap-absua-cai", // monitor
    "ca77u-aiaaa-aaaap-abxiq-cai", // dashboard
    // catalyze canisters
    "aqs24-xaaaa-aaaal-qbbea-cai", // catalyze frontend
    "e3q3a-7yaaa-aaaap-ab3qq-cai", // SIWE
    "mtcuh-kiaaa-aaaap-ahasa-cai", // SIWS
    // child production IDs
    "5nrjv-iaaaa-aaaap-aa4la-cai", // members
    "5rvte-7aaaa-aaaap-aa4ja-cai", // groups
    "4vy4w-gaaaa-aaaap-aa4pa-cai", // profiles
    "zocah-aqaaa-aaaap-aa4qa-cai", // events
    "zaanp-3aaaa-aaaap-aa4ra-cai", // event attendees
    "zsg2w-xqaaa-aaaap-aa4sa-cai", // reports
];
