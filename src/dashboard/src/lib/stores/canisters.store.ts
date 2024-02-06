import { sortedCanisterCycles } from "$lib/api/monitor.api"
import type { CanisterCycles } from "$lib/declarations/monitor.did"

import { writable } from "svelte/store"

const cycles = await sortedCanisterCycles()

export const canisterStore = writable<CanisterCycles[]>(cycles)
