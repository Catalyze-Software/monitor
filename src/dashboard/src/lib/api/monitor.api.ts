import { Principal } from "@dfinity/principal"
import { tryCall } from "$lib/utils/call.utils"
import { authStore } from "$lib/stores/auth.store"
import { Actor } from "@dfinity/agent"
import {
  idlFactory as monitor_idl,
  type _SERVICE,
  type CanisterCycles,
  type CycleBalances,
  type Log,
} from "$lib/declarations/monitor.did"
import { createAgent } from "@dfinity/utils"

const monitorCanisterId: Principal = Principal.fromText(
  "6or45-oyaaa-aaaap-absua-cai"
)

const monitorActor = async () => {
  const identity = await authStore.identity()
  const agent = await createAgent({
    identity,
    fetchRootKey: false,
  })
  return Actor.createActor<_SERVICE>(monitor_idl, {
    canisterId: monitorCanisterId,
    agent,
  })
}

export const latestIcpBalances = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], [bigint, number][]>(
    monitor.latest_icp_balances,
    n
  )
}

export const sortedCanisterCycles = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], CanisterCycles[]>(monitor.sorted_canister_cycles)
}

export const latestCycleBalances = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], CycleBalances[]>(
    monitor.latest_cycle_balances,
    n
  )
}

export const latestLogs = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], Log[]>(monitor.get_latest_logs, n)
}
