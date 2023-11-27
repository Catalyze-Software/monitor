use candid::Principal;
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::CanisterIdRecord,
};

use crate::store::CanisterCycles;

/*
* Canisters not present in `GetSnsCanistersSummaryResponse`
*
* Production principals:
*/
// pub const CHILD_MEMBERS: &str = "5nrjv-iaaaa-aaaap-aa4la-cai";
// pub const CHILD_GROUPS: &str = "5rvte-7aaaa-aaaap-aa4ja-cai";
// pub const CHILD_PROFILES: &str = "4vy4w-gaaaa-aaaap-aa4pa-cai";
// pub const CHILD_EVENTS: &str = "zocah-aqaaa-aaaap-aa4qa-cai";
// pub const CHILD_EVENT_ATTENDEES: &str = "zaanp-3aaaa-aaaap-aa4ra-cai";
// pub const CHILD_REPORTS: &str = "zsg2w-xqaaa-aaaap-aa4sa-cai";

// Development principals
pub const CHILD_MEMBERS: &str = "dogcy-xyaaa-aaaap-abqsa-cai";
pub const CHILD_GROUPS: &str = "cdigw-yqaaa-aaaap-abqvq-cai";
pub const CHILD_PROFILES: &str = "crorp-uaaaa-aaaap-abqwq-cai";
pub const CHILD_EVENTS: &str = "cnkl6-daaaa-aaaap-abquq-cai";
pub const CHILD_EVENT_ATTENDEES: &str = "dhfje-bqaaa-aaaap-abqtq-cai";
pub const CHILD_REPORTS: &str = "c7m4h-pqaaa-aaaap-abqxq-cai";

pub async fn get_child_canister_summary() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    vec.push(CanisterCycles::from_status(
        "child_members",
        CHILD_MEMBERS,
        &get_canister_status(CHILD_MEMBERS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "child_groups",
        CHILD_GROUPS,
        &get_canister_status(CHILD_GROUPS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "child_profiles",
        CHILD_PROFILES,
        &get_canister_status(CHILD_PROFILES).await,
    ));

    vec.push(CanisterCycles::from_status(
        "child_events",
        CHILD_EVENTS,
        &get_canister_status(CHILD_EVENTS).await,
    ));

    vec.push(CanisterCycles::from_status(
        "child_event_attendees",
        CHILD_EVENT_ATTENDEES,
        &get_canister_status(CHILD_EVENT_ATTENDEES).await,
    ));

    vec.push(CanisterCycles::from_status(
        "child_reports",
        CHILD_REPORTS,
        &get_canister_status(CHILD_REPORTS).await,
    ));

    vec
}

async fn get_canister_status(canister_id: &str) -> CanisterStatusResponse {
    let canister_id = Principal::from_text(canister_id).expect("Invalid principal");
    let arg = CanisterIdRecord { canister_id };

    canister_status(arg)
        .await
        .expect("Failed to call canister_status")
        .0
}
