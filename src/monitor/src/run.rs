use crate::{
    operations::{charge::top_up_canisters, read::take_snapshot},
    stores::stable_store::{CanisterStatusStore, Logs},
    utils::log::EVENT_COMPLETED_READ_OPERATION,
};

/*
* Perform a full run of state update and charge operations
*/
pub async fn run() {
    let snapshot = take_snapshot().await;

    top_up_canisters(&snapshot).await;

    CanisterStatusStore::insert(snapshot);

    Logs::log(EVENT_COMPLETED_READ_OPERATION.to_string());
}
