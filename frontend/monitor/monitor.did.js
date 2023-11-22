export const idlFactory = ({ IDL }) => {
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  return IDL.Service({
    'get_icp_balance' : IDL.Func([], [Tokens], []),
    'icp_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'last_poll_time' : IDL.Func([], [IDL.Nat64], ['query']),
    'ops' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
