import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterCycles {
  'name' : string,
  'canister_id' : Principal,
  'cycles' : bigint,
}
export interface _SERVICE {
  'get_log' : ActorMethod<[bigint], Array<string>>,
  'icp_balance' : ActorMethod<[], string>,
  'last_poll_time' : ActorMethod<[], string>,
  'sorted_canister_cycles' : ActorMethod<[], Array<CanisterCycles>>,
  'update_state' : ActorMethod<[], undefined>,
}
