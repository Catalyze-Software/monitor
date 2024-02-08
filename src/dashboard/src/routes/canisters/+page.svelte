<script lang="ts">
  import { sortedCanisterCycles } from "$lib/api/monitor.api"
  import CanisterStatus from "$lib/components/cards/CanisterStatus.svelte"
  import type { CanisterCycles } from "$lib/declarations/monitor.did"
  import { onMount } from "svelte"

  let ready = false
  let cycles: CanisterCycles[]

  onMount(async () => {
    cycles = await sortedCanisterCycles()
    ready = true
  })
</script>

{#if ready}
  <div class="wrapper">
    {#each cycles as canister}
      <div class="card">
        <CanisterStatus {canister} />
      </div>{/each}
  </div>
{:else}
  <p>Loading...</p>
{/if}

<style lang="scss">
  .wrapper {
    display: flex;
    align-items: center;
    justify-content: left;
    flex-wrap: wrap;
  }

  .card {
    margin: 0 1rem;
    width: 28rem;
  }
</style>
