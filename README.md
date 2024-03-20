# Monitor and dashboard

Canister status monitoring tool

The monitor backend is a Rust canister with stable storage stores. It can be upgraded without losing any data.
It fetches data from the canisters periodically and logs its fetching operation. During the fetch, it checks cycle balance for each canister and charges the canister if it is low on cycles.
The monitor backend provides several queries to fetch history logs, cycle balances, and other data.

The dashboard is a SvelteKit frontend, implemented as an asset canister on ICP. It fetches data from the monitor backend and displays it in a user-friendly way with graphs. It uses II to auth users. New users need to be added to `src/monitor/src/utils/auth.rs`. The principal is logged in the console when the user logs in.

## Before deplying

Read `src/monitor/src/system.rs`for info on the timer before deploying.

## Deploying

`dfx deploy --ic` will build and deploy both `monitor` and `dashboard`

To build and deploy a single canister, use `dfx deploy monitor --ic` or `dfx deploy dashboard --ic`

## Adding another canister to monitor

add canister data model to stable_models.rs and impl `Storable` for it

add canister data store to stable_store.rs and impl its store

add canister to operations and read.rs

add canister to sort.rs and cycle_stats.rs

IMPORTANT: initially, fill its store with 0 data to equal the size of the other stores, otherwise cycles_history.rs doesnt work
This can be done by writing a temporary update function that itereates over an existing store, taking the timestamps and fill the new store with these timestamps and 0 cycle balances
