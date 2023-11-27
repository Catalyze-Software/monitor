use candid::Principal;
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::CanisterIdRecord,
};

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

pub async fn get_child_canister_summary() -> Vec<(String, CanisterStatusResponse)> {
    let mut vec = Vec::new();

    vec.push((
        String::from("child_members"),
        get_canister_status(Principal::from_text(CHILD_MEMBERS).expect("Invalid principal")).await,
    ));

    vec.push((
        String::from("child_groups"),
        get_canister_status(Principal::from_text(CHILD_GROUPS).expect("Invalid principal")).await,
    ));

    vec.push((
        String::from("child_profiles"),
        get_canister_status(Principal::from_text(CHILD_PROFILES).expect("Invalid principal")).await,
    ));

    vec.push((
        String::from("child_events"),
        get_canister_status(Principal::from_text(CHILD_EVENTS).expect("Invalid principal")).await,
    ));

    vec.push((
        String::from("child_event_attendees"),
        get_canister_status(
            Principal::from_text(CHILD_EVENT_ATTENDEES).expect("Invalid principal"),
        )
        .await,
    ));

    vec.push((
        String::from("child_reports"),
        get_canister_status(Principal::from_text(CHILD_REPORTS).expect("Invalid principal")).await,
    ));

    vec
}

async fn get_canister_status(canister_id: Principal) -> CanisterStatusResponse {
    let arg = CanisterIdRecord { canister_id };

    canister_status(arg)
        .await
        .expect("Failed to call canister_status")
        .0
}
