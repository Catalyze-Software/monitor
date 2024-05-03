use crate::stores::{stable_store::CanisterStatusStore, types::Timestamp};
use candid::{CandidType, Deserialize, Nat};
use num_traits::ToPrimitive;

type CanisterName = String;
type TCycles = f64;

#[derive(CandidType, Deserialize)]
pub struct CycleBalances {
    timestamp: Timestamp,
    balances: Vec<(CanisterName, TCycles)>,
}

/*
* Cycle history data for all canisters
* n is the number of data points to return
* each data point (CycleBalances) contains the timestamp of the data point and the cycle balances of all canisters
*/
pub fn get_latest_cycle_balances(n: u64) -> Vec<CycleBalances> {
    let history_size = CanisterStatusStore::size();

    // ensure n is not greater than store size
    let mut n = n;
    if n > history_size {
        n = history_size;
    }

    let history = CanisterStatusStore::get_latest_n(n);

    let mut time_series = Vec::new();

    for snapshot in history {
        let timestamp = snapshot.timestamp;
        let mut balances = Vec::new();

        for canister in snapshot.canisters {
            let cycles = canister.status.cycles.expect("No cycles");
            let name = canister.canister_name;

            balances.push((name, cycles_to_tcycles(cycles)));
        }

        time_series.push(CycleBalances {
            timestamp,
            balances,
        });
    }

    time_series
}

fn cycles_to_tcycles(cycles: Nat) -> f64 {
    let t_cycles =
        cycles.0.to_f64().expect("failed to convert cycles to f64") / 1_000_000_000_000.0;

    // round to three decimal places
    (t_cycles * 1000.0).round() / 1000.0
}
