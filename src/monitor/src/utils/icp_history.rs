use crate::stores::{stable_models::Timestamp, stable_store::MonitorStore};

use super::tokens_to_icp;

pub fn get_latest_icp_balances(n: u64) -> Vec<(Timestamp, f64)> {
    let history = MonitorStore::get_latest_n(n);

    history
        .iter()
        .map(|monitor_data| {
            let timestamp = monitor_data.timestamp;
            let icp_balance = tokens_to_icp(monitor_data.icp_balance);
            (timestamp, icp_balance)
        })
        .collect()
}
