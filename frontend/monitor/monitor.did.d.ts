import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'all_cycle_balances' : ActorMethod<[], Array<string>>,
  'get_latest_with_timestamp' : ActorMethod<[bigint], Array<string>>,
  'get_log' : ActorMethod<[bigint], Array<string>>,
  'icp_balance' : ActorMethod<[], string>,
  'latest_icp_balances' : ActorMethod<[bigint], Array<string>>,
  'update_state' : ActorMethod<[], undefined>,
}
