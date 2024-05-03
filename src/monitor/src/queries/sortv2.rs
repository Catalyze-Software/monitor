use crate::stores::{stable_store::CanisterStatusStore, types::CanisterCycles};

/*
* Sorted canister cycles
* Get the latest cycle balances of all canisters and sort them in ascending order
*/
pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let latest_snapshot = CanisterStatusStore::get_latest().expect("No canister status found");

    let mut canister_cycles: Vec<CanisterCycles> = latest_snapshot
        .canisters
        .into_iter()
        .map(|canister_snapshot| CanisterCycles::from(canister_snapshot))
        .collect();

    canister_cycles.sort_by(|a, b| a.cycles.cmp(&b.cycles));

    canister_cycles
}
