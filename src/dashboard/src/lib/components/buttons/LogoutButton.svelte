<script>
  import { authStore } from "$lib/stores/auth.store"
  import { IconLogout } from "@dfinity/gix-components"
</script>

<button class="ghost" on:click={async () => await authStore.logout()}>
  <IconLogout />
</button>
