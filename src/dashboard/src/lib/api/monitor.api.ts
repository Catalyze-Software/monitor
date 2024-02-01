import { Principal } from "@dfinity/principal"
import { tryCall } from "$lib/utils/call.utils"
import { authStore } from "$lib/stores/auth.store"
import { Actor } from "@dfinity/agent"
import {
  idlFactory as monitor_idl,
  type _SERVICE,
} from "$lib/declarations/monitor.did"
import { createAgent } from "@dfinity/utils"

const monitorCanisterId: Principal = Principal.fromText(
  "6or45-oyaaa-aaaap-absua-cai"
)

const identity = await authStore.identity()

const agent = await createAgent({
  identity,
  fetchRootKey: false,
})

const monitor = Actor.createActor<_SERVICE>(monitor_idl, {
  canisterId: monitorCanisterId,
  agent,
})

export const latestIcpBalances = async (n: bigint) => {
  return await tryCall<[bigint], [bigint, number][]>(
    monitor.latest_icp_balances,
    n
  )
}
