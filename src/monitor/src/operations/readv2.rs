use super::sns::{CanisterStatusResultV2, CanisterStatusType as SNSCanisterStatusType};
use crate::{
    stores::{
        stable_store::{CanisterStatusStore, Logs, MonitorStore},
        types::MonitorICPBalance,
        types::{CanisterSnapshot, CatalyzeCanisterStatus, Snapshot},
    },
    utils::{
        canister_status::get_canister_status,
        log::{
            EVENT_CATALYZE_CANISTER_DATA, EVENT_COMPLETED_READ_OPERATION, EVENT_MONITOR_DATA,
            EVENT_SNS_DATA,
        },
    },
    CANISTER_IDS, CANISTER_NAMES,
};
use candid::{Nat, Principal};
use ic_cdk::api::{
    management_canister::main::{
        CanisterStatusResponse, CanisterStatusType, DefiniteCanisterSettings, QueryStats,
    },
    time,
};

/*
* Perform canister status query routine
*/
pub async fn read_operations() {
    // Monitor data read, store and log operations
    // Monitor canister is only canister for which we store icp balance
    let icp_balance = crate::operations::ledger::icp_balance().await;

    let monitor_data = MonitorICPBalance {
        timestamp: time(),
        icp_balance,
    };

    MonitorStore::insert(monitor_data);
    Logs::log(format!("{}", EVENT_MONITOR_DATA.to_string()));

    // Start global snapshot (SNS + Catalyze canisters)
    let mut snapshot = Snapshot {
        timestamp: time(),
        canisters: Vec::new(),
    };

    // SNS read, store and log operations
    let summary = crate::operations::sns::get_sns_canisters_summary().await;

    let root_canister_snapshot = CanisterSnapshot {
        canister_name: "SNS Root".to_string(),
        canister_id: summary.root.as_ref().unwrap().canister_id.unwrap().clone(),
        status: summary.root.unwrap().status.unwrap().into(),
    };

    snapshot.canisters.push(root_canister_snapshot);

    let swap_canister_snapshot = CanisterSnapshot {
        canister_name: "SNS Swap".to_string(),
        canister_id: summary.swap.as_ref().unwrap().canister_id.unwrap().clone(),
        status: summary.swap.unwrap().status.unwrap().into(),
    };

    snapshot.canisters.push(swap_canister_snapshot);

    let ledger_canister_snapshot = CanisterSnapshot {
        canister_name: "SNS Ledger".to_string(),
        canister_id: summary
            .ledger
            .as_ref()
            .unwrap()
            .canister_id
            .unwrap()
            .clone(),
        status: summary.ledger.unwrap().status.unwrap().into(),
    };

    snapshot.canisters.push(ledger_canister_snapshot);

    let index_canister_snapshot = CanisterSnapshot {
        canister_name: "SNS Index".to_string(),
        canister_id: summary.index.as_ref().unwrap().canister_id.unwrap().clone(),
        status: summary.index.unwrap().status.unwrap().into(),
    };

    snapshot.canisters.push(index_canister_snapshot);

    let governance_canister_snapshot = CanisterSnapshot {
        canister_name: "SNS Governance".to_string(),
        canister_id: summary
            .governance
            .as_ref()
            .unwrap()
            .canister_id
            .unwrap()
            .clone(),
        status: summary.governance.unwrap().status.unwrap().into(),
    };

    snapshot.canisters.push(governance_canister_snapshot);

    // iter over dapps and archives
    for (i, dapp) in summary.dapps.iter().enumerate() {
        let dapp_canister_snapshot = CanisterSnapshot {
            canister_name: format!("Dapps {}", i),
            canister_id: dapp.canister_id.unwrap().clone(),
            status: CatalyzeCanisterStatus::from(dapp.status.clone().unwrap()),
        };

        snapshot.canisters.push(dapp_canister_snapshot);
    }

    for (i, archive) in summary.archives.iter().enumerate() {
        let archive_canister_snapshot = CanisterSnapshot {
            canister_name: format!("Archives {}", i),
            canister_id: archive.canister_id.unwrap().clone(),
            status: CatalyzeCanisterStatus::from(archive.status.clone().unwrap()),
        };

        snapshot.canisters.push(archive_canister_snapshot);
    }

    Logs::log(format!("{}", EVENT_SNS_DATA.to_string()));

    // iter over catalyze canisters
    for (i, name) in CANISTER_NAMES.iter().enumerate() {
        let canister_status = get_canister_status(CANISTER_IDS[i]).await;

        let canister_snapshot = CanisterSnapshot {
            canister_name: name.to_string(),
            canister_id: Principal::from_text(CANISTER_IDS[i]).unwrap(),
            status: CatalyzeCanisterStatus::from(canister_status),
        };

        snapshot.canisters.push(canister_snapshot);
    }

    Logs::log(format!("{}", EVENT_CATALYZE_CANISTER_DATA.to_string()));

    // Store snapshot
    CanisterStatusStore::insert(snapshot);

    // Log snapshot
    Logs::log(format!("{}", EVENT_COMPLETED_READ_OPERATION.to_string()));
}

impl From<CanisterStatusResultV2> for CatalyzeCanisterStatus {
    fn from(value: CanisterStatusResultV2) -> Self {
        let status = match value.status {
            SNSCanisterStatusType::Stopped => CanisterStatusType::Stopped,
            SNSCanisterStatusType::Stopping => CanisterStatusType::Stopping,
            SNSCanisterStatusType::Running => CanisterStatusType::Running,
        };

        let memory_size = Nat::from(value.memory_size);

        let cycles = value.cycles;

        let settings = DefiniteCanisterSettings {
            freezing_threshold: value.settings.freezing_threshold,
            controllers: value.settings.controllers,
            memory_allocation: value.settings.memory_allocation,
            compute_allocation: value.settings.compute_allocation,
        };

        let idle_cycles_burned_per_day = value.idle_cycles_burned_per_day;

        // map Option<ByteBuf> to Option<Vec<u8>>
        let module_hash = value.module_hash.map(|byte_buf| byte_buf.to_vec());

        let query_stats: Option<QueryStats> = None;

        CatalyzeCanisterStatus {
            status: Some(status),
            memory_size: Some(memory_size),
            cycles: Some(cycles),
            settings: Some(settings),
            idle_cycles_burned_per_day: Some(idle_cycles_burned_per_day),
            module_hash,
            query_stats,
        }
    }
}

impl From<CanisterStatusResponse> for CatalyzeCanisterStatus {
    fn from(value: CanisterStatusResponse) -> Self {
        CatalyzeCanisterStatus {
            status: Some(value.status),
            settings: Some(value.settings),
            module_hash: value.module_hash,
            memory_size: Some(value.memory_size),
            cycles: Some(value.cycles),
            idle_cycles_burned_per_day: Some(value.idle_cycles_burned_per_day),
            query_stats: Some(value.query_stats),
        }
    }
}
