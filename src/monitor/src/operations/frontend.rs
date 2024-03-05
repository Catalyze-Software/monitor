use crate::stores::stable_models::CanisterCycles;

use crate::utils::canister_status::get_canister_status;

// Production principals:
pub const FRONTEND: &str = "aqs24-xaaaa-aaaal-qbbea-cai";

/*
* Frontend canister summary
*/
pub async fn get_frontend_canister_summary() -> CanisterCycles {
    CanisterCycles::from_status("Frontend", FRONTEND, &get_canister_status(FRONTEND).await)
}
