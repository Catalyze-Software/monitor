use crate::stores::{
    stable_models::CanisterCycles,
    stable_store::{ChildStore, MonitorStore, SnsStore},
};

/*
* Sorted canister cycles
*/
pub fn sorted_canister_cycles() -> Vec<CanisterCycles> {
    let mut vec = Vec::new();

    // add this monitor canister
    let monitor_data = MonitorStore::get_latest().unwrap();

    vec.push(CanisterCycles {
        name: String::from("Monitor"),
        canister_id: ic_cdk::id(),
        cycles: monitor_data.cycle_balance,
    });

    // add the SNS canisters
    let summary = SnsStore::get_latest().unwrap();

    vec.push(CanisterCycles::new("SNS Root", &summary.root.unwrap()));
    vec.push(CanisterCycles::new("SNS Swap", &summary.swap.unwrap()));
    vec.push(CanisterCycles::new("SNS Ledger", &summary.ledger.unwrap()));
    vec.push(CanisterCycles::new("SNS Index", &summary.index.unwrap()));
    vec.push(CanisterCycles::new(
        "SNS Governance",
        &summary.governance.unwrap(),
    ));

    // iterate over the dapps canisters
    for (i, canister) in summary.dapps.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("Dapps {}", i), &canister));
    }

    // iterate over the archives canisters
    for (i, canister) in summary.archives.iter().enumerate() {
        vec.push(CanisterCycles::new(&format!("Archives {}", i), &canister));
    }

    // add the child canisters
    let child_data = ChildStore::get_latest().unwrap();

    vec.push(child_data.members.clone());
    vec.push(child_data.groups.clone());
    vec.push(child_data.profiles.clone());
    vec.push(child_data.events.clone());
    vec.push(child_data.event_attendees.clone());
    vec.push(child_data.reports.clone());

    // sort the vec by cycles in ascending order
    vec.sort_by(|a, b| a.cycles.cmp(&b.cycles));

    vec
}

pub fn cycle_balances() -> Vec<String> {
    sorted_canister_cycles()
        .iter()
        .map(|canister| {
            format!(
                "{}: {}",
                canister.name,
                format!("{:.2}", canister.cycles.clone())
            )
        })
        .collect()
}
