use ic_ledger_types::Tokens;

pub mod cycle_history;
pub mod icp_history;
pub mod sortv2;

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
