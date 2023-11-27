use crate::{operations::sns::CanisterSummary, store::STATE};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::trap;

/*
* Select each canister-cycles pair from STATE
* and return a sorted vector of these pairs
*/
#[derive(CandidType, Deserialize)]
pub struct CanisterCycles {
    pub name: String,
    pub canister_id: Principal,
    pub cycles: Nat,
}

impl CanisterCycles {
    pub fn new<'a>(name: &str, canister_summary: &CanisterSummary) -> Self {
        Self {
            name: String::from(name),
            canister_id: canister_summary.canister_id.unwrap(),
            cycles: canister_summary.status.as_ref().unwrap().cycles.clone(),
        }
    }
}

pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    let summary = STATE.with(|s| s.borrow().get_summary());

    // get cycles for the general sns canisters
    vec.push(CanisterCycles::new("root", &summary.root.unwrap()));
    vec.push(CanisterCycles::new("swap", &summary.swap.unwrap()));
    vec.push(CanisterCycles::new("ledger", &summary.ledger.unwrap()));
    vec.push(CanisterCycles::new("index", &summary.index.unwrap()));
    vec.push(CanisterCycles::new(
        "governance",
        &summary.governance.unwrap(),
    ));

    // iterate over the dapps canisters
    for (i, canister) in summary.dapps.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("dapps {}", i), &canister));
    }

    // iterate over the archives canisters
    for (i, canister) in summary.archives.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("archives {}", i), &canister));
    }

    // iterate over child canisters (not present in `GetSnsCanistersSummaryResponse`)
    // STATE.with(|s| match s.borrow().childs.clone() {
    //     None => {}
    //     Some(childs) => {
    //         for (canister, status) in childs {
    //             vec.push(CanisterCycles(canister, status.cycles));
    //         }
    //     }
    // });

    // add this monitor canister cycle balance
    vec.push(CanisterCycles {
        name: String::from("monitor"),
        canister_id: ic_cdk::id(),
        cycles: STATE.with(|s| s.borrow().get_cycle_balance()),
    });

    // sort the vec by cycles in ascending order
    vec.sort_by(|a, b| a.cycles.cmp(&b.cycles));

    vec
}
