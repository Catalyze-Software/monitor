use crate::store::STATE;
use candid::{CandidType, Deserialize, Nat};
use ic_cdk::trap;

/*
* Select each canister-cycles pair from STATE
* and return a sorted vector of these pairs
*/
#[derive(CandidType, Deserialize)]
pub struct CanisterCycles(pub String, pub Nat);

pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    let summary = STATE.with(|s| s.borrow().get_summary());

    // get cycles for the general sns canisters
    vec.push(CanisterCycles(
        String::from("root"),
        summary
            .root
            .unwrap_or_else(|| trap("Root canister not found"))
            .status
            .unwrap_or_else(|| trap("Root canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("swap"),
        summary
            .swap
            .unwrap_or_else(|| trap("Swap canister not found"))
            .status
            .unwrap_or_else(|| trap("Swap canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("ledger"),
        summary
            .ledger
            .unwrap_or_else(|| trap("Ledger canister not found"))
            .status
            .unwrap_or_else(|| trap("Ledger canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("index"),
        summary
            .index
            .unwrap_or_else(|| trap("Index canister not found"))
            .status
            .unwrap_or_else(|| trap("Index canister status not found"))
            .cycles,
    ));

    vec.push(CanisterCycles(
        String::from("governance"),
        summary
            .governance
            .unwrap_or_else(|| trap("Governance canister not found"))
            .status
            .unwrap_or_else(|| trap("Governance canister status not found"))
            .cycles,
    ));

    // iterate over the dapps canisters
    for (i, canister) in summary.dapps.iter().enumerate() {
        vec.push(CanisterCycles(
            format!("dapps {}", i),
            canister
                .status
                .as_ref()
                .unwrap_or_else(|| trap("Dapps canister status not found"))
                .cycles
                .clone(),
        ));
    }

    // iterate over the archives canisters
    for (i, canister) in summary.archives.iter().enumerate() {
        vec.push(CanisterCycles(
            format!("archives {}", i),
            canister
                .status
                .as_ref()
                .unwrap_or_else(|| trap("Archives canister status not found"))
                .cycles
                .clone(),
        ));
    }

    // iterate over child canisters (not present in `GetSnsCanistersSummaryResponse`)
    STATE.with(|s| match s.borrow().childs.clone() {
        None => {}
        Some(childs) => {
            for (canister, status) in childs {
                vec.push(CanisterCycles(canister, status.cycles));
            }
        }
    });

    // add this monitor canister cycle balance
    vec.push(CanisterCycles(
        String::from("monitor"),
        STATE.with(|s| s.borrow().get_cycle_balance()),
    ));

    // sort the vec by cycles in ascending order
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec
}
