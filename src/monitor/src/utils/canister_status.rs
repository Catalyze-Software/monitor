use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse,
};

pub async fn get_canister_status(canister_id: &str) -> CanisterStatusResponse {
    let canister_id = Principal::from_text(canister_id).expect("Invalid principal");
    let arg = CanisterIdRecord { canister_id };

    canister_status(arg)
        .await
        .expect("Failed to call canister_status")
        .0
}
