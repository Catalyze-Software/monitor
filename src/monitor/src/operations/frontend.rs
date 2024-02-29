use crate::stores::stable_models::CanisterCycles;
use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse,
};

// Production principals:
pub const FRONTEND: &str = "aqs24-xaaaa-aaaal-qbbea-cai";

/*
* Frontend canister summary
*/
pub async fn get_frontend_canister_summary() -> CanisterCycles {
    CanisterCycles::from_status("Frontend", FRONTEND, &get_canister_status(FRONTEND).await)
}

async fn get_canister_status(canister_id: &str) -> CanisterStatusResponse {
    let canister_id = Principal::from_text(canister_id).expect("Invalid principal");
    let arg = CanisterIdRecord { canister_id };

    canister_status(arg)
        .await
        .expect("Failed to call canister_status")
        .0
}
