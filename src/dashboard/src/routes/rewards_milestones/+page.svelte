<script lang="ts">
  import { groupInfo, eventInfo } from "$lib/api/monitor.api";
  import type { GroupInfo } from "$lib/declarations/monitor.did";
  import { Card } from "@dfinity/gix-components";
  import { onMount } from "svelte";

  let ready = false;

  let groupInfos: GroupInfo[] = [];

  onMount(async () => {
    groupInfos = await groupInfo();

    ready = true;
  });
</script>

<h1>Reward milestones</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <h2>Group info</h2>
  <p>Total groups: {groupInfos.length}</p>
  <!-- <p>Total events: {eventInfos.length}</p> -->

  <h3>Group milestones</h3>
  {#each groupInfos as group}
    <Card>
      <p>Group owner: {group.owner}</p>
      <p>Group member count milestone: {group.count_milestone}</p>
      <p>Group activity milestone: {group.activity_milestone}</p>
    </Card>
  {/each}

  <!-- <h3>Event milestones</h3>
  {#each eventInfos as event}
    <Card>
      <p>Event owner: {event.owner}</p>
      <p>Event milestone: {event.attendance_milestone}</p>
    </Card>
  {/each} -->
{/if}
