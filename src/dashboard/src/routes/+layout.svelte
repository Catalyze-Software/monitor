<script>
  import { authStore } from "$lib/stores/auth.store"

  import { Content, Layout, MenuItem, Toasts } from "@dfinity/gix-components"

  import FrontPageBanner from "$lib/components/layout/FrontPageBanner.svelte"
  import LogoutButton from "$lib/components/buttons/LogoutButton.svelte"
  import Logo from "$lib/components/layout/Logo.svelte"

  import { goto } from "$app/navigation"
  import { onMount } from "svelte"

  onMount(async () => {
    let authed = authStore.isAuthenticated()
    if (!authed) {
      goto("/")
    }
  })
</script>

<Toasts />

<svelte:head>
  <title>Catalyze Dashboard</title>
</svelte:head>

<Layout>
  <Logo slot="menu-logo" />

  <svelte:fragment slot="menu-items">
    <MenuItem href="/" on:click>Overview</MenuItem>
    <MenuItem href="/cycles-history" on:click>Cycles history</MenuItem>
    <MenuItem href="/canisters" on:click>Canisters</MenuItem>
    <MenuItem href="/monitor_logs" on:click>Monitor logs</MenuItem>

    <div class="menu-spacer">
      <span>Proxy canister</span>
    </div>
    <MenuItem href="/proxy_logs" on:click>Proxy logs</MenuItem>
    <MenuItem href="/proxy_reward_buffer" on:click>Rewards buffer</MenuItem>

    <div class="menu-spacer">
      <span>Rewards canister</span>
    </div>
    <MenuItem href="/rewards_schedule" on:click>Rewards schedule</MenuItem>
    <MenuItem href="/rewards_token_balances" on:click>Token balances</MenuItem>
    <MenuItem href="/rewards_milestones" on:click>Reward milestones</MenuItem>
  </svelte:fragment>

  <Content>
    <LogoutButton slot="toolbar-end" />
    <main>
      {#if !$authStore}
        <FrontPageBanner />
      {:else if $authStore}
        <slot />
      {/if}
    </main>
  </Content>
</Layout>

<style lang="scss" global>
  @import "../../node_modules/@dfinity/gix-components/dist/styles/global.scss";

  .menu-spacer {
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--color-text-muted);
  }
</style>
