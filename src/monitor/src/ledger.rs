use crate::log::{EVENT_CYCLES_MINTED, EVENT_ICP_SENT};
use crate::store::log;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::time;
use ic_cdk::id;
use ic_cdk::{api::management_canister::main::canister_status, call};
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, BlockIndex, Memo, Subaccount,
    Timestamp, Tokens, TransferArgs, DEFAULT_SUBACCOUNT, MAINNET_CYCLES_MINTING_CANISTER_ID,
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

/*
Query the CMC canister for the ICP/XDR rate
*/
pub async fn icp_xdr_rate() -> IcpXdrConversionRate {
    call::<(), (IcpXdrConversionRateResponse,)>(
        MAINNET_CYCLES_MINTING_CANISTER_ID,
        "get_icp_xdr_conversion_rate",
        (),
    )
    .await
    .expect("Failed to call get_icp_to_xdr_rate")
    .0
    .data
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRateResponse {
    certificate: Vec<u8>,
    data: IcpXdrConversionRate,
    hash_tree: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
    xdr_permyriad_per_icp: u64,
    timestamp_seconds: u64,
}

/*
Transfer ICP to CMC canister for cycle minting
*/
pub async fn transfer_icp_to_cmc_for_cycles_minting(amount: Tokens) -> BlockIndex {
    let arg = TransferArgs {
        memo: MEMO_TOP_UP_CANISTER,
        amount,
        fee: ICP_TRANSACTION_FEE,
        from_subaccount: None,
        to: AccountIdentifier::new(&MAINNET_CYCLES_MINTING_CANISTER_ID, &Subaccount::from(id())),
        created_at_time: Some(Timestamp {
            timestamp_nanos: time(),
        }),
    };

    transfer(MAINNET_LEDGER_CANISTER_ID, arg)
        .await
        .expect("Failed to transfer ICP to CMC")
        .unwrap_or_else(|err| {
            ic_cdk::trap(&format!(
                "Failed to transfer ICP to CMC for cycle minting: {:?}",
                err
            ))
        })
}

pub static MEMO_TOP_UP_CANISTER: Memo = Memo(1347768404_u64);
pub static ICP_TRANSACTION_FEE: Tokens = Tokens::from_e8s(10000);

/*
Notify CMC of a ICP tx for cycle minting
*/
pub async fn notify_top_up(block_index: BlockIndex, canister_id: Principal) -> Cycles {
    let arg = NotifyTopUpArg {
        block_index,
        canister_id,
    };

    let notify_top_ip_result = call::<(NotifyTopUpArg,), (NotifyTopUpResult,)>(
        MAINNET_CYCLES_MINTING_CANISTER_ID,
        "notify_top_up",
        (arg,),
    )
    .await
    .expect("Failed to call notify_top_up")
    .0;

    match notify_top_ip_result {
        NotifyTopUpResult::Ok(cycles) => cycles,
        NotifyTopUpResult::Err(err) => {
            ic_cdk::trap(&format!("Failed to notify_top_up: {:?}", err));
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct NotifyTopUpArg {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

pub type Cycles = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(Cycles),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NotifyError {
    Refunded {
        block_index: Option<BlockIndex>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(BlockIndex),
}

const TOP_UP_XDR_AMOUNT_PERMYRIADS: u64 = 10;

pub async fn top_up_self() {
    let rate = icp_xdr_rate().await.xdr_permyriad_per_icp;
    let icp_e8s = (TOP_UP_XDR_AMOUNT_PERMYRIADS * 10000) / rate;
    log(format!(
        "ICP/XDR rate: {}, top up amount: {} ICP",
        rate,
        Tokens::from_e8s(icp_e8s)
    ));

    let block_index = transfer_icp_to_cmc_for_cycles_minting(Tokens::from_e8s(icp_e8s)).await;
    log(format!(
        "{}: {} ICP",
        EVENT_ICP_SENT,
        Tokens::from_e8s(icp_e8s)
    ));

    let cycles = notify_top_up(block_index, id()).await;
    log(format!(
        "{}: {} cycles for canister {}",
        EVENT_CYCLES_MINTED,
        cycles,
        id()
    ));
}
