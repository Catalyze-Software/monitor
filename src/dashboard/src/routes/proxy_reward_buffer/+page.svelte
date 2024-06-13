<script lang="ts">
  import { readProxyRewardBuffer, rewardTimerNextTrigger } from "$lib/api/monitor.api";
  import type { RewardableActivityResponse } from "$lib/api/proxy.declaration";
  import type { RewardableActivity } from "$lib/declarations/monitor.did";
  import { convertTimestampToDateTime } from "$lib/utils/date.utils";
  import { decode } from "@dfinity/candid/lib/cjs/idl";
  import { Card } from "@dfinity/gix-components";
  import { onMount } from "svelte";

  let ready = false;

  let rewardableActivities: { activity: string; id: string; timestamp: bigint }[] = [];
  let nextTrigger: string = "";

  onMount(async () => {
    rewardableActivities = await readProxyRewardBuffer();

    let rewardTimer = await rewardTimerNextTrigger();
    nextTrigger = rewardTimer[0] ? convertTimestampToDateTime(rewardTimer[0]) : "No next trigger";

    ready = true;
  });
</script>

<h1>Proxy reward buffer</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <p>Next trigger: {nextTrigger}</p>
  {#each rewardableActivities as activity}
    <Card>
      <h3 slot="start">{convertTimestampToDateTime(activity.timestamp)}</h3>
      <p>identifier: {activity.id}</p>
      <p>Activity: {activity.activity}</p>
    </Card>
  {/each}
{/if}
