import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterCycles {
  'name' : string,
  'canister_id' : Principal,
  'cycles' : bigint,
}
export interface CanisterMemorySize {
  'name' : string,
  'size' : bigint,
  'canister_id' : Principal,
}
export interface CycleHistory {
  'timestamps' : BigUint64Array | bigint[],
  'line_data' : Array<LineData>,
}
export interface LineData { 'cycles' : Array<number>, 'canister_name' : string }
export interface Log { 'msg' : string, 'timestamp' : bigint }
export interface Logger {
  'principal' : [] | [Principal],
  'source' : [] | [string],
  'data' : [] | [string],
  'description' : string,
  'created_on' : bigint,
}
export interface _SERVICE {
  'canister_cycle_history' : ActorMethod<[bigint], CycleHistory>,
  'get_latest_logs' : ActorMethod<[bigint], Array<Log>>,
  'get_latest_with_timestamp' : ActorMethod<[bigint], Array<string>>,
  'graph_event_attendee_rewards' : ActorMethod<[], Array<[bigint, bigint]>>,
  'graph_member_activity_rewards' : ActorMethod<[], Array<[bigint, bigint]>>,
  'graph_member_count_rewards' : ActorMethod<[], Array<[bigint, bigint]>>,
  'icp_balance' : ActorMethod<[], string>,
  'initiate_run' : ActorMethod<[], undefined>,
  'latest_icp_balances' : ActorMethod<[bigint], Array<[bigint, number]>>,
  'latest_proxy_logs' : ActorMethod<[bigint], Array<Logger>>,
  'proxy_log_size' : ActorMethod<[], bigint>,
  'sorted_canister_cycles' : ActorMethod<[], Array<CanisterCycles>>,
  'sorted_memory_sizes' : ActorMethod<[], Array<CanisterMemorySize>>,
  'store_stats' : ActorMethod<[], Array<string>>,
  'timer_set' : ActorMethod<[], boolean>,
  'token_balances' : ActorMethod<[], Array<[Principal, bigint]>>,
  'token_log_size' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
