use ic_cdk::id;
use ic_cdk_macros::update;
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
