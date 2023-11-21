import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterStatusResultV2 {
  'status' : CanisterStatusType,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettingsArgs,
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
}
export type CanisterStatusType = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export interface CanisterSummary {
  'status' : [] | [CanisterStatusResultV2],
  'canister_id' : [] | [Principal],
}
export interface DefiniteCanisterSettingsArgs {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface GetSnsCanistersSummaryResponse {
  'root' : [] | [CanisterSummary],
  'swap' : [] | [CanisterSummary],
  'ledger' : [] | [CanisterSummary],
  'index' : [] | [CanisterSummary],
  'governance' : [] | [CanisterSummary],
  'dapps' : Array<CanisterSummary>,
  'archives' : Array<CanisterSummary>,
}
export interface _SERVICE {
  'get_sns_canisters_summary_test' : ActorMethod<
    [],
    GetSnsCanistersSummaryResponse
  >,
}
