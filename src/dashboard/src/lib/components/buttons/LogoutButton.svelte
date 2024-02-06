<script>
  import { authStore } from "$lib/stores/auth.store"
  import { IconLogout } from "@dfinity/gix-components"
</script>

<div class="logoutButton">
  <button class="ghost" on:click={async () => await authStore.logout()}>
    <IconLogout />
  </button>
</div>

<style lang="scss">
  .logoutButton {
    display: flex;
    align-items: center;
    justify-content: right;
    margin-right: 2rem;
  }
</style>
