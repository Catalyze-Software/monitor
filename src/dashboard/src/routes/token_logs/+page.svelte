<script lang="ts">
  import { latestTokenRewards } from "$lib/api/monitor.api"
  import type { RewardData } from "$lib/declarations/monitor.did"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Card } from "@dfinity/gix-components"
  import { onMount } from "svelte"

  let ready = false

  let rewards: RewardData[] = []

  onMount(async () => {
    rewards = await latestTokenRewards(100n)
    ready = true
  })
</script>

{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  {#each rewards as reward}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(reward.timestamp)}</h3>
      <p>{reward.description}</p>
      <p>{reward.principal.toText()}</p>
    </Card>
  {/each}
{/if}
