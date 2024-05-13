export const idlFactory = ({ IDL }) => {
  const LineData = IDL.Record({
    'cycles' : IDL.Vec(IDL.Float64),
    'canister_name' : IDL.Text,
  });
  const CycleHistory = IDL.Record({
    'timestamps' : IDL.Vec(IDL.Nat64),
    'line_data' : IDL.Vec(LineData),
  });
  const Log = IDL.Record({ 'msg' : IDL.Text, 'timestamp' : IDL.Nat64 });
  const Logger = IDL.Record({
    'principal' : IDL.Opt(IDL.Principal),
    'source' : IDL.Opt(IDL.Text),
    'data' : IDL.Opt(IDL.Text),
    'description' : IDL.Text,
    'created_on' : IDL.Nat64,
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
  const RewardData = IDL.Record({
    'principal' : IDL.Principal,
    'description' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  return IDL.Service({
    'canister_cycle_history' : IDL.Func([IDL.Nat64], [CycleHistory], ['query']),
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
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'initiate_run' : IDL.Func([], [], []),
    'latest_icp_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
        ['query'],
      ),
    'latest_proxy_logs' : IDL.Func([IDL.Nat64], [IDL.Vec(Logger)], []),
    'proxy_log_size' : IDL.Func([], [IDL.Nat64], []),
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
    'token_latest_rewards' : IDL.Func([IDL.Nat64], [IDL.Vec(RewardData)], []),
    'token_log_size' : IDL.Func([], [IDL.Nat64], []),
  });
};
export const init = ({ IDL }) => { return []; };
