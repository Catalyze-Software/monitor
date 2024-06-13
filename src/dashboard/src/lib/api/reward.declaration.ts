import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export interface Error {
  tag: [] | [string];
  info: [] | [Array<string>];
  method_name: [] | [string];
  message: [] | [string];
  timestamp: bigint;
  error_type: ErrorKind;
}
export type ErrorKind =
  | { Internal: null }
  | { Duplicate: null }
  | { SerializeError: null }
  | { DeserializeError: null }
  | { InsufficientBalance: null }
  | { NotFound: null }
  | { Unsupported: null }
  | { Unauthorized: null }
  | { NotImplemented: null }
  | { BadRequest: null };
export interface GroupInfo {
  activity_milestone: bigint;
  owner: Principal;
  count_milestone: bigint;
}
export type Result = { Ok: Principal } | { Err: Error };
export interface RewardData {
  id: bigint;
  owner: Principal;
  count: bigint;
}
export interface UserInfo {
  activity_milestone: bigint;
  owner: Principal;
}
export interface _SERVICE {
  _dev_get_proxy: ActorMethod<[], Result>;
  _dev_set_proxy: ActorMethod<[Principal], boolean>;
  add_and_process_proxy_buffer: ActorMethod<[Array<RewardData>, Array<RewardData>], undefined>;
  graph_event_attendee_rewards: ActorMethod<[], Array<[bigint, bigint]>>;
  graph_member_activity_rewards: ActorMethod<[], Array<[bigint, bigint]>>;
  graph_member_count_rewards: ActorMethod<[], Array<[bigint, bigint]>>;
  group_info: ActorMethod<[], Array<GroupInfo>>;
  icts_name: ActorMethod<[], string>;
  icts_version: ActorMethod<[], string>;
  initiate_calc_rewards: ActorMethod<[], undefined>;
  is_timer_set: ActorMethod<[], boolean>;
  log_size: ActorMethod<[], bigint>;
  token_balances: ActorMethod<[], Array<[Principal, bigint]>>;
  user_info: ActorMethod<[], Array<UserInfo>>;
}
export const idlFactory = ({ IDL }: any) => {
  const ErrorKind = IDL.Variant({
    Internal: IDL.Null,
    Duplicate: IDL.Null,
    SerializeError: IDL.Null,
    DeserializeError: IDL.Null,
    InsufficientBalance: IDL.Null,
    NotFound: IDL.Null,
    Unsupported: IDL.Null,
    Unauthorized: IDL.Null,
    NotImplemented: IDL.Null,
    BadRequest: IDL.Null,
  });
  const Error = IDL.Record({
    tag: IDL.Opt(IDL.Text),
    info: IDL.Opt(IDL.Vec(IDL.Text)),
    method_name: IDL.Opt(IDL.Text),
    message: IDL.Opt(IDL.Text),
    timestamp: IDL.Nat64,
    error_type: ErrorKind,
  });
  const Result = IDL.Variant({ Ok: IDL.Principal, Err: Error });
  const RewardData = IDL.Record({
    id: IDL.Nat64,
    owner: IDL.Principal,
    count: IDL.Nat64,
  });
  const GroupInfo = IDL.Record({
    activity_milestone: IDL.Nat64,
    owner: IDL.Principal,
    count_milestone: IDL.Nat64,
  });
  const UserInfo = IDL.Record({
    activity_milestone: IDL.Nat64,
    owner: IDL.Principal,
  });
  return IDL.Service({
    _dev_get_proxy: IDL.Func([], [Result], ["query"]),
    _dev_set_proxy: IDL.Func([IDL.Principal], [IDL.Bool], []),
    add_and_process_proxy_buffer: IDL.Func([IDL.Vec(RewardData), IDL.Vec(RewardData)], [], []),
    graph_event_attendee_rewards: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))], ["query"]),
    graph_member_activity_rewards: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))], ["query"]),
    graph_member_count_rewards: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))], ["query"]),
    group_info: IDL.Func([], [IDL.Vec(GroupInfo)], ["query"]),
    icts_name: IDL.Func([], [IDL.Text], ["query"]),
    icts_version: IDL.Func([], [IDL.Text], ["query"]),
    initiate_calc_rewards: IDL.Func([], [], []),
    is_timer_set: IDL.Func([], [IDL.Bool], ["query"]),
    log_size: IDL.Func([], [IDL.Nat64], ["query"]),
    token_balances: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))], ["query"]),
    user_info: IDL.Func([], [IDL.Vec(UserInfo)], ["query"]),
  });
};
