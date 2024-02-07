<script lang="ts">
  import { latestLogs } from "$lib/api/monitor.api"
  import type { Log } from "$lib/declarations/monitor.did"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Content } from "@dfinity/gix-components"
  import { onMount } from "svelte"

  let ready = false

  let logs: Log[] = []

  onMount(async () => {
    logs = await latestLogs(1000n)
    ready = true
  })
</script>

<Content>
  <h1 slot="title">Monitor logs</h1>
  {#each logs as log}
    <div class="log">
      <strong>{convertTimestampToDateTime(log.timestamp)}</strong>
      {log.msg}
    </div>
  {/each}
</Content>

<style>
  .log {
    background-color: #f5f5f5;
    padding: 0.5rem;
    margin: 0.5rem;
  }

  .log strong {
    color: #666;
    display: block;
  }
</style>
