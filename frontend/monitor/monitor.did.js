export const idlFactory = ({ IDL }) => {
  const CanisterStatusType = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const DefiniteCanisterSettingsArgs = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatusResultV2 = IDL.Record({
    'status' : CanisterStatusType,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettingsArgs,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CanisterSummary = IDL.Record({
    'status' : IDL.Opt(CanisterStatusResultV2),
    'canister_id' : IDL.Opt(IDL.Principal),
  });
  const GetSnsCanistersSummaryResponse = IDL.Record({
    'root' : IDL.Opt(CanisterSummary),
    'swap' : IDL.Opt(CanisterSummary),
    'ledger' : IDL.Opt(CanisterSummary),
    'index' : IDL.Opt(CanisterSummary),
    'governance' : IDL.Opt(CanisterSummary),
    'dapps' : IDL.Vec(CanisterSummary),
    'archives' : IDL.Vec(CanisterSummary),
  });
  return IDL.Service({
    'get_sns_canisters_summary_test' : IDL.Func(
        [],
        [GetSnsCanistersSummaryResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
