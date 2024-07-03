<script lang="ts">
  import { groupInfo, userInfo } from "$lib/api/monitor.api";
  import type { GroupInfo, UserInfo } from "$lib/api/reward.declaration";
  import { Card } from "@dfinity/gix-components";
  import { onMount } from "svelte";

  let ready = false;

  let groupInfos: GroupInfo[] = [];
  let userInfos: UserInfo[] = [];

  onMount(async () => {
    groupInfos = await groupInfo();
    userInfos = await userInfo();

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
      <p>Group member count milestone: {group.member_count_milestone}</p>
      <p>Group activity milestone: {group.activity_milestone}</p>
    </Card>
  {/each}

  <h3>User milestones</h3>
  {#each userInfos as user}
    <Card>
      <p>User login streak: {Number(user.login_day_streak)}</p>
      <p>logins count: {Number(user.activity_history_nanos.length)}</p>
    </Card>
  {/each}
{/if}
