<script lang="ts">
  import { proxyStoreStats } from "$lib/api/monitor.api"
  import { onMount } from "svelte"

  let ready = false

  let stats: string[] = []

  onMount(async () => {
    stats = await proxyStoreStats()

    ready = true
  })
</script>

<h1>Proxy canister stats</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <h3>Proxy canister store sizes</h3>
  {#each stats as stat}
    <p>{stat}</p>
  {/each}
{/if}
