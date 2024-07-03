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
  member_count_milestone: bigint;
}
export interface GroupReward {
  group_member_count: bigint;
  owner: Principal;
  group_id: bigint;
}
export type Result = { Ok: Principal } | { Err: Error };
export type Result_1 = { Ok: null } | { Err: Error };
export type Result_2 = { Ok: [bigint, GroupInfo] } | { Err: Error };
export type Result_3 = { Ok: [Principal, UserInfo] } | { Err: Error };
export type Result_4 = { Ok: [Principal, bigint] } | { Err: Error };
export interface UserActivity {
  owner: Principal;
  timestamp: bigint;
}
export interface UserInfo {
  activity_history_nanos: BigUint64Array | bigint[];
  login_day_streak: bigint;
}
export interface UserLoginHistory {
  activity_history_nanos: BigUint64Array | bigint[];
}
export interface _SERVICE {
  __get_candid_interface_tmp_hack: ActorMethod<[], string>;
  _dev_clear: ActorMethod<[], undefined>;
  _dev_get_proxy: ActorMethod<[], Result>;
  _dev_prod_init: ActorMethod<[], Result_1>;
  _dev_set_proxy: ActorMethod<[Principal], boolean>;
  all_group_info: ActorMethod<[], Array<GroupInfo>>;
  all_token_balances: ActorMethod<[], Array<[Principal, bigint]>>;
  all_user_activity: ActorMethod<[], Array<[Principal, UserLoginHistory]>>;
  all_user_info: ActorMethod<[], Array<UserInfo>>;
  get_group_info: ActorMethod<[bigint], Result_2>;
  get_user_info: ActorMethod<[Principal], Result_3>;
  get_user_points: ActorMethod<[Principal], Result_4>;
  graph_member_activity_rewards: ActorMethod<[], Array<[bigint, bigint]>>;
  graph_member_count_rewards: ActorMethod<[], Array<[bigint, bigint]>>;
  icts_name: ActorMethod<[], string>;
  icts_version: ActorMethod<[], string>;
  initiate_calc_rewards: ActorMethod<[], undefined>;
  is_timer_set: ActorMethod<[], boolean>;
  log_size: ActorMethod<[], bigint>;
  process_buffer: ActorMethod<[Array<GroupReward>, Array<UserActivity>], undefined>;
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
  const Result_1 = IDL.Variant({ Ok: IDL.Null, Err: Error });
  const GroupInfo = IDL.Record({
    activity_milestone: IDL.Nat64,
    owner: IDL.Principal,
    member_count_milestone: IDL.Nat64,
  });
  const UserLoginHistory = IDL.Record({
    activity_history_nanos: IDL.Vec(IDL.Nat64),
  });
  const UserInfo = IDL.Record({
    activity_history_nanos: IDL.Vec(IDL.Nat64),
    login_day_streak: IDL.Nat64,
  });
  const Result_2 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Nat64, GroupInfo),
    Err: Error,
  });
  const Result_3 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Principal, UserInfo),
    Err: Error,
  });
  const Result_4 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Principal, IDL.Nat64),
    Err: Error,
  });
  const GroupReward = IDL.Record({
    group_member_count: IDL.Nat64,
    owner: IDL.Principal,
    group_id: IDL.Nat64,
  });
  const UserActivity = IDL.Record({
    owner: IDL.Principal,
    timestamp: IDL.Nat64,
  });
  return IDL.Service({
    __get_candid_interface_tmp_hack: IDL.Func([], [IDL.Text], ["query"]),
    _dev_clear: IDL.Func([], [], []),
    _dev_get_proxy: IDL.Func([], [Result], ["query"]),
    _dev_prod_init: IDL.Func([], [Result_1], []),
    _dev_set_proxy: IDL.Func([IDL.Principal], [IDL.Bool], []),
    all_group_info: IDL.Func([], [IDL.Vec(GroupInfo)], ["query"]),
    all_token_balances: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))], ["query"]),
    all_user_activity: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Principal, UserLoginHistory))], ["query"]),
    all_user_info: IDL.Func([], [IDL.Vec(UserInfo)], ["query"]),
    get_group_info: IDL.Func([IDL.Nat64], [Result_2], ["query"]),
    get_user_info: IDL.Func([IDL.Principal], [Result_3], ["query"]),
    get_user_points: IDL.Func([IDL.Principal], [Result_4], ["query"]),
    graph_member_activity_rewards: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))], ["query"]),
    graph_member_count_rewards: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))], ["query"]),
    icts_name: IDL.Func([], [IDL.Text], ["query"]),
    icts_version: IDL.Func([], [IDL.Text], ["query"]),
    initiate_calc_rewards: IDL.Func([], [], []),
    is_timer_set: IDL.Func([], [IDL.Bool], ["query"]),
    log_size: IDL.Func([], [IDL.Nat64], ["query"]),
    process_buffer: IDL.Func([IDL.Vec(GroupReward), IDL.Vec(UserActivity)], [], []),
  });
};
