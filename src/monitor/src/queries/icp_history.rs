use super::tokens_to_icp;
use crate::stores::{stable_models::Timestamp, stable_store::MonitorStore};

pub fn get_latest_icp_balances(n: u64) -> Vec<(Timestamp, f64)> {
    // double n because we skip even indexes
    let mut n = n * 2;

    // ensure monitor store has enough logs
    let monitor_size = MonitorStore::size();
    if n > monitor_size {
        n = monitor_size;
    }
    
    let history = MonitorStore::get_latest_n(n);

    let mut time_series = Vec::new();

    for i in 0..n as usize {
        // read operations happen twice every run, see run.rs
        // to get only the last read operation of each run (read after charging finished)
        // we skip even indexes
        if i % 2 == 0 {
            continue;
        }

        let timestamp = history[i].timestamp;
        let icp_balance = tokens_to_icp(history[i].icp_balance);

        time_series.push((timestamp, icp_balance));
    }

    time_series
}
