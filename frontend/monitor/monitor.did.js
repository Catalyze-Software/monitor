export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'cycle_balance' : IDL.Func([], [IDL.Nat], ['query']),
    'get_log' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Text)], ['query']),
    'icp_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'last_poll_time' : IDL.Func([], [IDL.Text], ['query']),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat))],
        ['query'],
      ),
    'test_top_up' : IDL.Func([], [], []),
    'update_state' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
