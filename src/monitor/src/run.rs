use crate::{
    operations::{charge::top_up_canisters, child::get_child_canister_summary},
    stores::{
        stable_models::{ChildData, MonitorData},
        stable_store::{ChildStore, Logs, MonitorStore, SnsStore},
    },
    utils::log::{EVENT_CHILD_SUMMARY, EVENT_MONITOR_DATA, EVENT_SNS_DATA},
};

/*
* Perform a full run of state update and charge operations
*/
pub async fn run() {
    read_operations().await;
    Logs::log("Read operations successful".to_string());

    top_up_canisters().await;
    Logs::log("Top up canisters successful".to_string());

    read_operations().await;
    Logs::log("Read operations successful".to_string());

    Logs::log("RUN COMPLETED SUCCESFULLY".to_string());
}

/*
* Perform canister status query routine
*/
pub async fn read_operations() {
    // Monitor read, store and log operations
    let icp_balance = crate::operations::ledger::icp_balance().await;
    let cycle_balance = crate::operations::cmc::cycle_balance().await;

    let monitor_data = MonitorData {
        icp_balance,
        cycle_balance,
    };

    MonitorStore::insert(monitor_data);
    Logs::log(format!("{}", EVENT_MONITOR_DATA.to_string()));

    // SNS read, store and log operations
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    SnsStore::insert(summary);
    Logs::log(format!("{}", EVENT_SNS_DATA.to_string()));

    // Child read, store and log operations
    let childs = get_child_canister_summary().await;
    let child_data = ChildData {
        members: childs[0].clone(),
        groups: childs[1].clone(),
        profiles: childs[2].clone(),
        events: childs[3].clone(),
        event_attendees: childs[4].clone(),
        reports: childs[5].clone(),
    };

    ChildStore::insert(child_data);
    Logs::log(format!("{}", EVENT_CHILD_SUMMARY.to_string()));
}
