use crate::{stores::stable_models::CanisterCycles, utils::canister_status::get_canister_status};

// Production principals:
pub const CHILD_MEMBERS: &str = "5nrjv-iaaaa-aaaap-aa4la-cai";
pub const CHILD_GROUPS: &str = "5rvte-7aaaa-aaaap-aa4ja-cai";
pub const CHILD_PROFILES: &str = "4vy4w-gaaaa-aaaap-aa4pa-cai";
pub const CHILD_EVENTS: &str = "zocah-aqaaa-aaaap-aa4qa-cai";
pub const CHILD_EVENT_ATTENDEES: &str = "zaanp-3aaaa-aaaap-aa4ra-cai";
pub const CHILD_REPORTS: &str = "zsg2w-xqaaa-aaaap-aa4sa-cai";

/*
* Canisters not present in `GetSnsCanistersSummaryResponse`
*/
pub async fn get_child_canister_summary() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    vec.push(CanisterCycles::from_status(
        "Members",
        CHILD_MEMBERS,
        &get_canister_status(CHILD_MEMBERS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "Groups",
        CHILD_GROUPS,
        &get_canister_status(CHILD_GROUPS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "Profiles",
        CHILD_PROFILES,
        &get_canister_status(CHILD_PROFILES).await,
    ));

    vec.push(CanisterCycles::from_status(
        "Events",
        CHILD_EVENTS,
        &get_canister_status(CHILD_EVENTS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "Event attendees",
        CHILD_EVENT_ATTENDEES,
        &get_canister_status(CHILD_EVENT_ATTENDEES).await,
    ));

    vec.push(CanisterCycles::from_status(
        "Reports",
        CHILD_REPORTS,
        &get_canister_status(CHILD_REPORTS).await,
    ));

    vec
}
