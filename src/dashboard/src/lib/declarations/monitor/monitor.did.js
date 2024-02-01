export const idlFactory = ({ IDL }) => {
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
    'store_stats' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
