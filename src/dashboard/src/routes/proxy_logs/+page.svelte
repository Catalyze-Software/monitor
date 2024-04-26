<script lang="ts">
  import { latestProxyLogs } from "$lib/api/monitor.api"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Card } from "@dfinity/gix-components"
  import { onMount } from "svelte"
  import type { Logger } from "$lib/declarations/monitor.did"

  let ready = false

  let logs: Logger[] = []

  onMount(async () => {
    logs = await latestProxyLogs(100n)
    ready = true
  })
</script>

{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  {#each logs as log}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(log.created_on)}</h3>
      <p>{log.description}</p>
    </Card>
  {/each}
{/if}
