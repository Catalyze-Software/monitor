use crate::{
    operations::{charge::top_up_canisters, readv2::read_operations},
    stores::stable_store::Logs,
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
