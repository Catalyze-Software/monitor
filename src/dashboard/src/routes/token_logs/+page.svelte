<script lang="ts">
  import { latestTokenRewards, tokenLogSize } from "$lib/api/monitor.api"
  import type { RewardData } from "$lib/declarations/monitor.did"
  import { convertTimestampToDateTime } from "$lib/utils/date.utils"
  import { Card } from "@dfinity/gix-components"
  import { onMount } from "svelte"

  let ready = false

  let rewards: RewardData[] = []
  let logSize: bigint | undefined = undefined

  onMount(async () => {
    rewards = await latestTokenRewards(100n)
    logSize = await tokenLogSize()

    ready = true
  })
</script>

<h1>Token canister logs</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <p>Log size: {logSize}</p>
  {#each rewards as reward}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(reward.timestamp)}</h3>
      <p>{reward.description}</p>
      <p>{reward.principal.toText()}</p>
    </Card>
  {/each}
{/if}
