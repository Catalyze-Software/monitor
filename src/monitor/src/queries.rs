use crate::stores::{
    stable_store::{CanisterStatusStore, MonitorStore},
    types::{CanisterCycles, CycleBalances, Snapshot, Timestamp},
};
use candid::Nat;
use ic_ledger_types::Tokens;
use num_traits::cast::ToPrimitive;

/*
* Sort canister cycles in snapshot
*/
pub fn sorted_canister_cycles(snapshot: &Snapshot) -> Vec<CanisterCycles> {
    let mut canister_cycles: Vec<CanisterCycles> = snapshot
        .canisters
        .iter()
        .map(|canister_snapshot| CanisterCycles::from(canister_snapshot.clone()))
        .collect();

    canister_cycles.sort_by(|a, b| a.cycles.cmp(&b.cycles));

    canister_cycles
}

/*
* Cycle history data for all canisters
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

pub fn get_latest_icp_balances(n: u64) -> Vec<(Timestamp, f64)> {
    // ensure monitor store has enough logs
    let mut n = n;
    let monitor_size = MonitorStore::size();
    if n > monitor_size {
        n = monitor_size;
    }

    let history = MonitorStore::get_latest_n(n);

    let mut time_series = Vec::new();

    for i in 0..n as usize {
        let timestamp = history[i].timestamp;
        let icp_balance = tokens_to_icp(history[i].icp_balance);

        time_series.push((timestamp, icp_balance));
    }

    time_series
}

pub fn range(n: u64, len: u64) -> (u64, u64) {
    // check length of tree to avoid underflow
    let (start, end) = if n > len {
        (1, len)
    } else {
        // ensures we get the last n logs
        (len - n + 1, len)
    };

    (start, end)
}

pub fn tokens_to_icp(tokens: Tokens) -> f64 {
    let balance = tokens.e8s() as f64 / (Tokens::SUBDIVIDABLE_BY as f64);

    // round to three decimal places
    (balance * 1000.0).round() / 1000.0
}
