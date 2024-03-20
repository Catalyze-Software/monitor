# Monitor

Canister status monitoring tool

## adding another canister to monitor

add canister data model to stable_models.rs and impl `Storable` for it

add canister data store to stable_store.rs and impl its store

add canister to operations and read.rs

add canister to sort.rs and cycle_stats.rs

IMPORTANT: initially, fill its store with 0 data to equal the size of the other stores, otherwise cycles_history.rs doesnt work
This can be done by writing a temporary update function that itereates over an existing store, taking the timestamps and fill the new store with these timestamps and 0 cycle balances
