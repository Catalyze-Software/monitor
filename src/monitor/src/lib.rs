mod api;
mod canisters;
mod operations;
mod queries;
mod run;
mod stores;
mod system;
mod utils;

/*
 * Canister names and IDs need to be in same order
 * This list contains all canisters that need to be fetched manually
 * This list excludes SNS canisters which are fetched at once with `get_sns_canisters_summary`
 */

// Canister names
pub const CANISTER_NAMES: [&str; 3] = [
    // monitor and dashboard
    "Monitor",
    "Dashboard",
    // catalyze canisters
    "Frontend",
];
// Canister list
pub const CANISTER_IDS: [&str; 3] = [
    // monitor and dashboard
    "6or45-oyaaa-aaaap-absua-cai", // monitor
    "ca77u-aiaaa-aaaap-abxiq-cai", // dashboard
    // catalyze canisters
    "h4xrq-fiaaa-aaaal-ajecq-cai", // catalyze frontend
];
