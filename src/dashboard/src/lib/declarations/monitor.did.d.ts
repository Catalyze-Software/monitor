import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterCycles {
  'name' : string,
  'canister_id' : Principal,
  'cycles' : bigint,
}
export interface CycleBalances {
  'timestamp' : bigint,
  'balances' : Array<[string, number]>,
}
export interface Log { 'msg' : string, 'timestamp' : bigint }
export interface Logger {
  'principal' : [] | [Principal],
  'source' : [] | [string],
  'data' : [] | [string],
  'description' : string,
  'created_on' : bigint,
}
export interface _SERVICE {
  'all_cycle_balances' : ActorMethod<[], Array<string>>,
  'get_latest_logs' : ActorMethod<[bigint], Array<Log>>,
  'get_latest_with_timestamp' : ActorMethod<[bigint], Array<string>>,
  'icp_balance' : ActorMethod<[], string>,
  'initiate_run' : ActorMethod<[], undefined>,
  'latest_cycle_balances' : ActorMethod<[bigint], Array<CycleBalances>>,
  'latest_icp_balances' : ActorMethod<[bigint], Array<[bigint, number]>>,
  'latest_proxy_logs' : ActorMethod<[bigint], Array<Logger>>,
  'new_user' : ActorMethod<[], [] | [Principal]>,
  'proxy_log_size' : ActorMethod<[], bigint>,
  'sorted_canister_cycles' : ActorMethod<[], Array<CanisterCycles>>,
  'store_stats' : ActorMethod<[], Array<string>>,
  'timer_set' : ActorMethod<[], boolean>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
