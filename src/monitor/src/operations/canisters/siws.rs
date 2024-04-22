use crate::stores::stable_models::CanisterCycles;

use crate::utils::canister_status::get_canister_status;

// Production principals:
pub const SIWS_CANISTER_ID: &str = "mtcuh-kiaaa-aaaap-ahasa-cai";

/*
* Siws canister summary
*/
pub async fn get_siws_canister_summary() -> CanisterCycles {
    CanisterCycles::from_status(
        "Siws",
        SIWS_CANISTER_ID,
        &get_canister_status(SIWS_CANISTER_ID).await,
    )
}
