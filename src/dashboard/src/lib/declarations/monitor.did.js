export const idlFactory = ({ IDL }) => {
  const LineData = IDL.Record({
    'cycles' : IDL.Vec(IDL.Float64),
    'canister_name' : IDL.Text,
  });
  const CycleHistory = IDL.Record({
    'timestamps' : IDL.Vec(IDL.Nat64),
    'line_data' : IDL.Vec(LineData),
  });
  const EventInfo = IDL.Record({
    'owner' : IDL.Principal,
    'attendance_milestone' : IDL.Nat64,
  });
  const Log = IDL.Record({ 'msg' : IDL.Text, 'timestamp' : IDL.Nat64 });
  const GroupInfo = IDL.Record({
    'activity_milestone' : IDL.Nat64,
    'owner' : IDL.Principal,
    'count_milestone' : IDL.Nat64,
  });
  const Logger = IDL.Record({
    'principal' : IDL.Opt(IDL.Principal),
    'source' : IDL.Opt(IDL.Text),
    'data' : IDL.Opt(IDL.Text),
    'description' : IDL.Text,
    'created_on' : IDL.Nat64,
  });
  const RewardableActivity = IDL.Record({
    'id' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
    'activity' : IDL.Text,
  });
  const CanisterCycles = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'cycles' : IDL.Nat,
  });
  const CanisterMemorySize = IDL.Record({
    'name' : IDL.Text,
    'size' : IDL.Nat,
    'canister_id' : IDL.Principal,
  });
  return IDL.Service({
    'canister_cycle_history' : IDL.Func([IDL.Nat64], [CycleHistory], ['query']),
    'event_info' : IDL.Func([], [IDL.Vec(EventInfo)], []),
    'get_latest_logs' : IDL.Func([IDL.Nat64], [IDL.Vec(Log)], ['query']),
    'get_latest_with_timestamp' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'graph_event_attendee_rewards' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
        [],
      ),
    'graph_member_activity_rewards' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
        [],
      ),
    'graph_member_count_rewards' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Nat64))],
        [],
      ),
    'group_info' : IDL.Func([], [IDL.Vec(GroupInfo)], []),
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'initiate_run' : IDL.Func([], [], []),
    'latest_icp_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
        ['query'],
      ),
    'latest_proxy_logs' : IDL.Func([IDL.Nat64], [IDL.Vec(Logger)], []),
    'proxy_log_size' : IDL.Func([], [IDL.Nat64], []),
    'read_reward_buffer' : IDL.Func([], [IDL.Vec(RewardableActivity)], []),
    'reward_timer_next_trigger' : IDL.Func([], [IDL.Opt(IDL.Nat64)], []),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(CanisterCycles)],
        ['query'],
      ),
    'sorted_memory_sizes' : IDL.Func(
        [],
        [IDL.Vec(CanisterMemorySize)],
        ['query'],
      ),
    'store_stats' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'timer_set' : IDL.Func([], [IDL.Bool], ['query']),
    'token_balances' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))],
        [],
      ),
    'token_log_size' : IDL.Func([], [IDL.Nat64], []),
  });
};
export const init = ({ IDL }) => { return []; };
