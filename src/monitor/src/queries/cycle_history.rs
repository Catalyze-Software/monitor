use crate::stores::{
    stable_models::Timestamp,
    stable_store::{ChildStore, FrontendStore, MonitorStore, SiweStore, SnsStore},
};
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
    // double n because we skip even indexes
    let mut n = n * 2;

    // ensure all stores have the same size
    let monitor_size = MonitorStore::size();
    let sns_size = SnsStore::size();
    let child_size = ChildStore::size();
    let frontend_size = FrontendStore::size();
    let siwe_size = SiweStore::size();

    assert_eq!(monitor_size, sns_size);
    assert_eq!(monitor_size, child_size);
    assert_eq!(monitor_size, frontend_size);
    assert_eq!(monitor_size, siwe_size);

    // ensure n is not greater than store size
    if n > monitor_size {
        n = monitor_size;
    }

    let monitor_history = MonitorStore::get_latest_n(n);
    let sns_history = SnsStore::get_latest_n(n);
    let child_history = ChildStore::get_latest_n(n);
    let frontend_history = FrontendStore::get_latest_n(n);
    let siwe_history = SiweStore::get_latest_n(n);

    let mut time_series = Vec::new();

    for i in 0..n as usize {
        // read operations happen twice every run, see run.rs
        // to get only the last read operation of each run (read after charging finished)
        // we skip even indexes
        if i % 2 == 0 {
            continue;
        }

        let mut balances = Vec::new();
        // timestamps should be the same for all stores
        let timestamp = monitor_history[i].timestamp;

        let monitor_balance = monitor_history[i].cycle_balance.clone();
        balances.push(("Monitor".to_string(), cycles_to_tcycles(monitor_balance)));

        // sns root
        let sns_root_balance = sns_history[i]
            .root
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .cycles
            .clone();
        balances.push(("SNS Root".to_string(), cycles_to_tcycles(sns_root_balance)));

        // sns swap
        let sns_swap_balance = sns_history[i]
            .swap
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .cycles
            .clone();
        balances.push(("SNS Swap".to_string(), cycles_to_tcycles(sns_swap_balance)));

        // sns ledger
        let sns_ledger_balance = sns_history[i]
            .ledger
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .cycles
            .clone();
        balances.push((
            "SNS Ledger".to_string(),
            cycles_to_tcycles(sns_ledger_balance),
        ));

        // sns index
        let sns_index_balance = sns_history[i]
            .index
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .cycles
            .clone();
        balances.push((
            "SNS Index".to_string(),
            cycles_to_tcycles(sns_index_balance),
        ));

        // sns governance
        let sns_governance_balance = sns_history[i]
            .governance
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .cycles
            .clone();
        balances.push((
            "SNS Governance".to_string(),
            cycles_to_tcycles(sns_governance_balance),
        ));

        // iter over sns dapps
        for (j, dapp) in sns_history[i].dapps.iter().enumerate() {
            let sns_dapp_balance = dapp.status.as_ref().unwrap().cycles.clone();
            balances.push((
                format!("SNS Dapp {}", j),
                cycles_to_tcycles(sns_dapp_balance),
            ));
        }

        // iter over sns archives
        for (j, archive) in sns_history[i].archives.iter().enumerate() {
            let sns_archive_balance = archive.status.as_ref().unwrap().cycles.clone();
            balances.push((
                format!("SNS Archive {}", j),
                cycles_to_tcycles(sns_archive_balance),
            ));
        }

        // add child canisters
        let members_balance = child_history[i].members.cycles.clone();
        balances.push(("Members".to_string(), cycles_to_tcycles(members_balance)));

        let groups_balance = child_history[i].groups.cycles.clone();
        balances.push(("Groups".to_string(), cycles_to_tcycles(groups_balance)));

        let profiles_balance = child_history[i].profiles.cycles.clone();
        balances.push(("Profiles".to_string(), cycles_to_tcycles(profiles_balance)));

        let events_balance = child_history[i].events.cycles.clone();
        balances.push(("Events".to_string(), cycles_to_tcycles(events_balance)));

        let event_attendees_balance = child_history[i].event_attendees.cycles.clone();
        balances.push((
            "Event Attendees".to_string(),
            cycles_to_tcycles(event_attendees_balance),
        ));

        let reports_balance = child_history[i].reports.cycles.clone();
        balances.push(("Reports".to_string(), cycles_to_tcycles(reports_balance)));

        // add frontend canister
        let frontend_balance = frontend_history[i].frontend.cycles.clone();
        balances.push(("Frontend".to_string(), cycles_to_tcycles(frontend_balance)));

        // add siwe canister
        let siwe_balance = siwe_history[i].siwe.cycles.clone();
        balances.push(("Siwe".to_string(), cycles_to_tcycles(siwe_balance)));

        // finally, add all balances at this timestamp (n=i) to time_series
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
