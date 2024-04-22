use crate::stores::stable_models::CanisterCycles;

use crate::utils::canister_status::get_canister_status;

// Production principals:
pub const DASHBOARD_CANISTER_ID: &str = "ca77u-aiaaa-aaaap-abxiq-cai";

/*
* Frontend canister summary
*/
pub async fn get_dashboard_canister_summary() -> CanisterCycles {
    CanisterCycles::from_status(
        "Dashboard",
        DASHBOARD_CANISTER_ID,
        &get_canister_status(DASHBOARD_CANISTER_ID).await,
    )
}
