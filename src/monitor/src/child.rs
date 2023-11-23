use crate::principals::{
    CHILD_EVENTS, CHILD_EVENT_ATTENDEES, CHILD_GROUPS, CHILD_MEMBERS, CHILD_PROFILES, CHILD_REPORTS,
};
use candid::Principal;
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::CanisterIdRecord,
};

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
