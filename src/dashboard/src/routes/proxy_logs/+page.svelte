<script lang="ts">
  import { latestProxyLogs, proxyLogSize } from "$lib/api/monitor.api"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Card } from "@dfinity/gix-components"
  import { onMount } from "svelte"
  import type { Logger } from "$lib/declarations/monitor.did"

  let ready = false

  let logs: Logger[] = []
  let logSize: bigint | undefined = undefined

  onMount(async () => {
    logs = await latestProxyLogs(100n)
    logSize = await proxyLogSize()

    ready = true
  })
</script>

<h1>Proxy canister logs</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <p>Log size: {logSize}</p>
  {#each logs as log}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(log.created_on)}</h3>
      <p>{log.description}</p>
    </Card>
  {/each}
{/if}
