export const idlFactory = ({ IDL }) => {
  const CanisterCycles = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'cycles' : IDL.Nat,
  });
  return IDL.Service({
    'all_cycle_balances' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_latest_with_timestamp' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'initiate_run' : IDL.Func([], [], []),
    'latest_cycle_balances' : IDL.Func(
        [IDL.Nat64],
        [
          IDL.Vec(
            IDL.Tuple(IDL.Nat64, IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float64)))
          ),
        ],
        ['query'],
      ),
    'latest_icp_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
        ['query'],
      ),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(CanisterCycles)],
        ['query'],
      ),
    'store_stats' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
