import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'icp_balance' : ActorMethod<[], bigint>,
  'last_poll_time' : ActorMethod<[], string>,
  'sorted_canister_cycles' : ActorMethod<[], Array<[string, bigint]>>,
}
