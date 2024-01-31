<script>
  import { Content, Layout, MenuItem } from "@dfinity/gix-components"
</script>

<Layout>
  <svelte:fragment slot="menu-logo"><span></span></svelte:fragment>
  <svelte:fragment slot="menu-oneliner"><span></span></svelte:fragment>
  <svelte:fragment slot="menu-items">
    <MenuItem href="/" on:click>Overview</MenuItem>
    <MenuItem href="/canisters" on:click>Canisters</MenuItem>
    <MenuItem href="/logs" on:click>Logs</MenuItem>
  </svelte:fragment>
  <Content>
    <h1 slot="title">Catalyze Dashboard</h1>
    <main>
      <slot />
    </main>
  </Content>
</Layout>

<style lang="scss" global>
  @import "../../node_modules/@dfinity/gix-components/dist/styles/global.scss";
</style>
