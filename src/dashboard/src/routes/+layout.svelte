<script>
  import { Content, Layout, MenuItem, Toasts } from "@dfinity/gix-components"
  import { authStore } from "$lib/stores/auth.store"
  import FrontPageBanner from "$lib/components/layout/FrontPageBanner.svelte"
  import LogoutButton from "$lib/components/buttons/LogoutButton.svelte"
  import Logo from "$lib/components/layout/Logo.svelte"
  import { onMount } from "svelte"
  import { goto } from "$app/navigation"

  onMount(() => {
    if (!authStore) {
      goto("/")
    }
  })
</script>

<Toasts />

<svelte:head>
  <title>Catalyze Dahsboard</title>
</svelte:head>

<Layout>
  <Logo slot="menu-logo" />
  <svelte:fragment slot="menu-items">
    <MenuItem href="/" on:click>Overview</MenuItem>
    <MenuItem href="/canisters" on:click>Canisters</MenuItem>
    <MenuItem href="/logs" on:click>Logs</MenuItem>
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
</style>
