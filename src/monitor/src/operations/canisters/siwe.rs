use crate::stores::stable_models::CanisterCycles;

use crate::utils::canister_status::get_canister_status;

// Production principals:
pub const SIWE_CANISTER_ID: &str = "e3q3a-7yaaa-aaaap-ab3qq-cai";

/*
* Siwe canister summary
*/
pub async fn get_siwe_canister_summary() -> CanisterCycles {
    CanisterCycles::from_status(
        "Siwe",
        SIWE_CANISTER_ID,
        &get_canister_status(SIWE_CANISTER_ID).await,
    )
}
