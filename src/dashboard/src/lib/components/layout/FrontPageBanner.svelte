<script lang="ts">
  import { authStore } from "$lib/stores/auth.store"
  import { IconSettingsPage, PageBanner } from "@dfinity/gix-components"
</script>

<PageBanner>
  <IconSettingsPage slot="image" />
  <span slot="title">Catalyze Dashboard</span>
  <p slot="description">Metrtics, graphs and more!</p>
  <button
    class="primary"
    slot="actions"
    id="loginButton"
    on:click={async () => await authStore.login()}
    >Connect with Internet Identity</button
  >
</PageBanner>
