import { sortedCanisterCycles } from "$lib/api/monitor.api"
import type { CanisterCycles } from "$lib/declarations/monitor.did"
import { readable } from "svelte/store"

export const initStore = async () => {
  const cycles = await sortedCanisterCycles()

  return readable<CanisterCycles[]>(cycles)
}

export const canisterStore = await initStore()