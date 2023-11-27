export const idlFactory = ({ IDL }) => {
  const CanisterCycles = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'cycles' : IDL.Nat,
  });
  return IDL.Service({
    'get_log' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Text)], ['query']),
    'icp_balance' : IDL.Func([], [IDL.Text], ['query']),
    'last_poll_time' : IDL.Func([], [IDL.Text], ['query']),
    'sorted_canister_cycles' : IDL.Func(
        [],
        [IDL.Vec(CanisterCycles)],
        ['query'],
      ),
    'update_state' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
