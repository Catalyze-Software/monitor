import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterCycles {
  'name' : string,
  'canister_id' : Principal,
  'cycles' : bigint,
}
export interface _SERVICE {
  'get_latest_with_timestamp' : ActorMethod<[bigint], Array<string>>,
  'get_log' : ActorMethod<[bigint], Array<string>>,
  'sorted_canister_cycles' : ActorMethod<[], Array<CanisterCycles>>,
  'update_state' : ActorMethod<[], undefined>,
}
