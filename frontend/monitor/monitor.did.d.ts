import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'cycle_balance' : ActorMethod<[], bigint>,
  'get_log' : ActorMethod<[bigint], Array<string>>,
  'icp_balance' : ActorMethod<[], bigint>,
  'last_poll_time' : ActorMethod<[], string>,
  'sorted_canister_cycles' : ActorMethod<[], Array<[string, bigint]>>,
  'test_top_up' : ActorMethod<[], undefined>,
  'update_state' : ActorMethod<[], undefined>,
}
