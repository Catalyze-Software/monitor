export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'icp_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'last_poll_time' : IDL.Func([], [IDL.Nat64], ['query']),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat))],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
