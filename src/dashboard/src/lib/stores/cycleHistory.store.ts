import { latestCycleBalances } from "$lib/api/monitor.api"
import type { CycleBalances } from "$lib/declarations/monitor.did"
import { readable } from "svelte/store"

const initStore = async () => {
  const latest = await latestCycleBalances(1000n)

  return readable<CycleBalances[]>(latest)
}

export const cycleHistoryStore = await initStore()