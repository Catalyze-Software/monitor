export const idlFactory = ({ IDL }) => {
  const Log = IDL.Record({ 'msg' : IDL.Text, 'timestamp' : IDL.Nat64 });
  const CycleBalances = IDL.Record({
    'timestamp' : IDL.Nat64,
    'balances' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float64)),
  });
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
  return IDL.Service({
    'all_cycle_balances' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_latest_logs' : IDL.Func([IDL.Nat64], [IDL.Vec(Log)], ['query']),
    'get_latest_with_timestamp' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'initiate_run' : IDL.Func([], [], []),
    'latest_cycle_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(CycleBalances)],
        ['query'],
      ),
    'latest_icp_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
        ['query'],
      ),
    'latest_proxy_logs' : IDL.Func([IDL.Nat64], [IDL.Vec(Logger)], []),
    'new_user' : IDL.Func([], [IDL.Opt(IDL.Principal)], ['query']),
    'proxy_log_size' : IDL.Func([], [IDL.Nat64], []),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(CanisterCycles)],
        ['query'],
      ),
    'store_stats' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'timer_set' : IDL.Func([], [IDL.Bool], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
