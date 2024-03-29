use super::{
    canisters::frontend::get_frontend_canister_summary, canisters::siwe::get_siwe_canister_summary,
};
use crate::{
    operations::canisters::child::get_child_canister_summary,
    stores::{
        stable_models::{ChildData, FrontendData, MonitorData, SiweData},
        stable_store::{ChildStore, FrontendStore, Logs, MonitorStore, SiweStore, SnsStore},
    },
    utils::log::{
        EVENT_CHILD_SUMMARY, EVENT_FRONTEND_SUMMARY, EVENT_MONITOR_DATA, EVENT_SIWE_SUMMARY,
        EVENT_SNS_DATA,
    },
};
use ic_cdk::api::time;

/*
* Perform canister status query routine
*/
pub async fn read_operations() {
    // Monitor read, store and log operations
    let icp_balance = crate::operations::ledger::icp_balance().await;
    let cycle_balance = crate::operations::cmc::cycle_balance().await;

    let monitor_data = MonitorData {
        timestamp: time(),
        icp_balance,
        cycle_balance,
    };

    MonitorStore::insert(monitor_data);
    Logs::log(format!("{}", EVENT_MONITOR_DATA.to_string()));

    // SNS read, store and log operations
    let summary = crate::operations::canisters::sns::get_sns_canisters_summary().await;

    SnsStore::insert(summary);
    Logs::log(format!("{}", EVENT_SNS_DATA.to_string()));

    // Child read, store and log operations
    let childs = get_child_canister_summary().await;
    let child_data = ChildData {
        timestamp: time(),
        members: childs[0].clone(),
        groups: childs[1].clone(),
        profiles: childs[2].clone(),
        events: childs[3].clone(),
        event_attendees: childs[4].clone(),
        reports: childs[5].clone(),
    };

    ChildStore::insert(child_data);
    Logs::log(format!("{}", EVENT_CHILD_SUMMARY.to_string()));

    // Frontend canister
    let frontend = get_frontend_canister_summary().await;
    let frontend_data = FrontendData {
        timestamp: time(),
        frontend,
    };
    FrontendStore::insert(frontend_data);
    Logs::log(format!("{}", EVENT_FRONTEND_SUMMARY.to_string()));

    // Siwe canister
    let siwe = get_siwe_canister_summary().await;
    let siwe_data = SiweData {
        timestamp: time(),
        siwe,
    };
    SiweStore::insert(siwe_data);
    Logs::log(format!("{}", EVENT_SIWE_SUMMARY.to_string()));
}
