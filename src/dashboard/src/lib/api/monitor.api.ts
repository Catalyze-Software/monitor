import { Principal } from "@dfinity/principal"
import { tryCall } from "$lib/utils/call.utils"
import { authStore } from "$lib/stores/auth.store"
import { Actor } from "@dfinity/agent"
import {
  idlFactory as monitor_idl,
  type _SERVICE,
  type CanisterCycles,
  type CanisterMemorySize,
  type CycleHistory,
  type Log,
  type Logger,
  type RewardData,
} from "$lib/declarations/monitor.did.js"
import { createAgent } from "@dfinity/utils"

const monitorCanisterId: Principal = Principal.fromText(
  "6or45-oyaaa-aaaap-absua-cai"
)

const monitorActor = async () => {
  const identity = await authStore.identity()
  const agent = await createAgent({
    identity,
    fetchRootKey: false,
    host: "https://icp-api.io",
  })
  return Actor.createActor<_SERVICE>(monitor_idl, {
    canisterId: monitorCanisterId,
    agent,
  })
}

export const icpBalance = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], string>(monitor.icp_balance)
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

export const sortedMemorySizes = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], CanisterMemorySize[]>(monitor.sorted_memory_sizes)
}

export const canisterCycleHistory = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], CycleHistory>(
    monitor.canister_cycle_history,
    n
  )
}

export const latestLogs = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], Log[]>(monitor.get_latest_logs, n)
}

export const latestProxyLogs = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], Logger[]>(monitor.latest_proxy_logs, n)
}

export const proxyLogSize = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], bigint>(monitor.proxy_log_size)
}

export const latestTokenRewards = async (n: bigint) => {
  const monitor = await monitorActor()
  return await tryCall<[bigint], RewardData[]>(monitor.token_latest_rewards, n)
}

export const tokenBalances = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], [Principal, bigint][]>(monitor.token_balances)
}

export const tokenLogSize = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], bigint>(monitor.token_log_size)
}

export const graphMemberCountRewards = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], [bigint, bigint][]>(
    monitor.graph_member_count_rewards
  )
}

export const graphMemberActivityRewards = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], [bigint, bigint][]>(
    monitor.graph_member_activity_rewards
  )
}

export const graphEventAttendeeRewards = async () => {
  const monitor = await monitorActor()
  return await tryCall<[], [bigint, bigint][]>(
    monitor.graph_event_attendee_rewards
  )
}
