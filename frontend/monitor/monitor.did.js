export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'all_cycle_balances' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_latest_with_timestamp' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'get_log' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Text)], ['query']),
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'latest_icp_balances' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'update_state' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
