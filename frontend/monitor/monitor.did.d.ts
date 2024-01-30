import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'all_cycle_balances' : ActorMethod<[], Array<string>>,
  'get_latest_with_timestamp' : ActorMethod<[bigint], Array<string>>,
  'icp_balance' : ActorMethod<[], string>,
  'initiate_run' : ActorMethod<[], undefined>,
  'latest_cycle_balances' : ActorMethod<
    [bigint],
    Array<[bigint, Array<[string, number]>]>
  >,
  'latest_icp_balances' : ActorMethod<[bigint], Array<[bigint, number]>>,
  'store_stats' : ActorMethod<[], Array<string>>,
}
