use crate::{
    operations::{charge::top_up_canisters, read::take_snapshot},
    stores::{
        stable_store::{CanisterStatusStore, Logs, MonitorStore},
        types::MonitorICPBalance,
    },
    utils::log::{EVENT_COMPLETED_ALL_OPERATION, EVENT_MONITOR_DATA},
};

/*
* Perform a full run of state update and charge operations
*/
pub async fn run() {
    // Read all canister statuses
    let snapshot = take_snapshot().await;

    // Top up canisters with low cycles
    let topped_up = top_up_canisters(&snapshot).await;

    // If any canisters were topped up, fetch snapshot again, otherwise store the same snapshot
    if topped_up {
        let new_snapshot = take_snapshot().await;
        CanisterStatusStore::insert(new_snapshot);
    } else {
        CanisterStatusStore::insert(snapshot);
    }

    // Monitor data read, store and log operations
    // Monitor canister is only canister for which we store icp balance
    // We update ICP balance after charge operations
    let icp_balance = crate::canisters::ledger::icp_balance().await;
    let monitor_data = MonitorICPBalance {
        timestamp: ic_cdk::api::time(),
        icp_balance,
    };
    MonitorStore::insert(monitor_data);
    Logs::log(EVENT_MONITOR_DATA.to_string());

    // Log completion of all operations
    Logs::log(EVENT_COMPLETED_ALL_OPERATION.to_string());
}
