use candid::Nat;
use ic_cdk::api::management_canister::main::canister_status;
use ic_cdk::id;
use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

/*
Query the ledger canister for the icp balance of the default account of this monitor canister
*/
pub async fn icp_balance() -> Tokens {
    let arg = AccountBalanceArgs {
        account: AccountIdentifier::new(&id(), &DEFAULT_SUBACCOUNT),
    };

    let balance = account_balance(MAINNET_LEDGER_CANISTER_ID, arg)
        .await
        .expect("Failed to query ledger canister for balance");

    balance
}

/*
Query the management canister for the cycle balance this monitor canister
*/
pub async fn cycle_balance() -> Nat {
    canister_status(
        ic_cdk::api::management_canister::provisional::CanisterIdRecord { canister_id: id() },
    )
    .await
    .expect("Failed to call canister_status")
    .0
    .cycles
}
