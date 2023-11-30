use super::cmc::{icp_xdr_rate, notify_top_up};
use super::ledger::transfer_icp_to_cmc_for_cycles_minting;
use crate::log::log;
use crate::log::{EVENT_CYCLES_MINTED, EVENT_ICP_SENT};
use crate::sort::sorted_canister_cycles;
use crate::store::{CanisterCycles, STATE};
use candid::Principal;
use ic_ledger_types::Tokens;

const CYCLES_BALANCES_THRESHOLD: u64 = 3_000_000_000_000;
const TOP_UP_XDR_AMOUNT_PERMYRIADS: u64 = 10_000_000; // 0.1 XDR = 10_000_000 permyriads

/*
* Iterate over the sorted SNS canister-cycles vector and top up canisters with low cycles
*/
pub async fn top_up_sns_canisters() {
    let sorted_canister_cycles = sorted_canister_cycles();

    for CanisterCycles {
        name,
        canister_id,
        cycles,
    } in sorted_canister_cycles
    {
        if cycles < CYCLES_BALANCES_THRESHOLD {
            top_up(name, canister_id).await;
        // since this vector is sorted in ascending cycle order, we can break early
        } else {
            break;
        }
    }
}

/*
* Iterate over the child canisters and top up canisters with low cycles
*/
pub async fn top_up_child_canisters() {
    let childs = STATE.with(|s| s.borrow().get_childs().unwrap_or_else(|| vec![]));

    for CanisterCycles {
        name,
        canister_id,
        cycles,
    } in childs
    {
        if cycles < CYCLES_BALANCES_THRESHOLD {
            top_up(name, canister_id).await;
        }
    }
}

/*
* Perform all async operations for topping up one canister with cycles and log results
*/
async fn top_up(name: String, canister_id: Principal) {
    // get rate, calculate icp amount and log result
    let rate = icp_xdr_rate().await.xdr_permyriad_per_icp;
    let icp_e8s = (TOP_UP_XDR_AMOUNT_PERMYRIADS * 10000) / rate;

    log(format!(
        "ICP/XDR rate: {}, top up amount: {} ICP",
        rate,
        Tokens::from_e8s(icp_e8s)
    ));

    // transfer icp to CMC for cycles minting and log result
    let block_index = transfer_icp_to_cmc_for_cycles_minting(Tokens::from_e8s(icp_e8s)).await;
    log(format!(
        "{}: {} ICP",
        EVENT_ICP_SENT,
        Tokens::from_e8s(icp_e8s)
    ));

    // notify CMC of the ICP tx and log result
    let cycles = notify_top_up(block_index, canister_id).await;
    log(format!(
        "{}: {} cycles for canister '{}' with canister_id {}",
        EVENT_CYCLES_MINTED, cycles, name, canister_id
    ));
}
