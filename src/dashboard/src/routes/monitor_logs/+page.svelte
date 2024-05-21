<script lang="ts">
  import { latestLogs } from "$lib/api/monitor.api"
  import type { Log } from "$lib/declarations/monitor.did"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Card } from "@dfinity/gix-components"
  import { onMount } from "svelte"

  let ready = false

  let logs: Log[] = []

  onMount(async () => {
    logs = await latestLogs(20n)
    ready = true
  })
</script>

<h1>Monitor canister logs</h1>
{#if !ready}
  <p>Performing query...</p>
{:else}
  {#each logs as log}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(log.timestamp)}</h3>
      <p>{log.msg}</p>
    </Card>
  {/each}
{/if}
