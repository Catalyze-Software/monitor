use candid::Principal;
use ic_ledger_types::Tokens;

use crate::{
    stores::{stable_models::CanisterCycles, stable_store::Logs},
    utils::{
        log::{EVENT_CYCLES_MINTED, EVENT_ICP_SENT},
        sort::sorted_canister_cycles,
    },
};

use super::{
    cmc::{icp_xdr_rate, notify_top_up},
    ledger::transfer_icp_to_cmc_for_cycles_minting,
};

const CYCLES_BALANCE_THRESHOLD: u64 = 6_500_000_000_000; // 5T
const CYCLE_TOP_UP_AMOUNT: u64 = 1_000_000_000;

/*
* Iterate over all canister-cycles vector and top up canisters with low cycles
*/
pub async fn top_up_canisters() {
    let sorted_canister_cycles = sorted_canister_cycles();

    for CanisterCycles {
        name,
        canister_id,
        cycles,
    } in sorted_canister_cycles
    {
        if cycles < CYCLES_BALANCE_THRESHOLD {
            top_up(name, canister_id).await;
        // since this vector is sorted in ascending cycle order, we can break early
        } else {
            break;
        }
    }
}

/*
* Perform all async operations for topping up one canister with cycles and log results
*/
async fn top_up(name: String, canister_id: Principal) {
    // get rate, calculate icp amount and log result
    let rate = icp_xdr_rate().await.xdr_permyriad_per_icp;
    let icp_e8s = CYCLE_TOP_UP_AMOUNT / rate;

    Logs::log(format!(
        "ICP/XDR rate: {}, top up amount: {} ICP",
        rate,
        Tokens::from_e8s(icp_e8s)
    ));

    // transfer icp to CMC for cycles minting and log result
    let block_index =
        transfer_icp_to_cmc_for_cycles_minting(Tokens::from_e8s(icp_e8s), canister_id).await;

    Logs::log(format!(
        "{}: {} ICP",
        EVENT_ICP_SENT,
        Tokens::from_e8s(icp_e8s)
    ));

    // notify CMC of the ICP tx and log result
    let cycles = notify_top_up(block_index, canister_id).await;

    Logs::log(format!(
        "{}: {} cycles for canister '{}' with canister_id {}",
        EVENT_CYCLES_MINTED, cycles, name, canister_id
    ));
}
