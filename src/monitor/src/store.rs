use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::sns::GetSnsCanistersSummaryResponse;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(Default)]
pub struct State {
    pub last_poll_time: u64,
    pub icp_balance: u64,
    pub summary: GetSnsCanistersSummaryResponse,
}

thread_local! {
    // static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    //     RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // static MAP: RefCell<StableBTreeMap<u128, u128, Memory>> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    //     )
    // );

    pub static STATE: RefCell<State> = RefCell::new(State::default());
}
