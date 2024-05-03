use super::tokens_to_icp;
use crate::stores::{stable_store::MonitorStore, types::Timestamp};

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
