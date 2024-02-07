<script lang="ts">
  import CanisterStatus from "$lib/components/cards/CanisterStatus.svelte"
  import { canisterStore } from "$lib/stores/canisters.store"
</script>

<div class="wrapper">
  {#each $canisterStore as canister}
    <div class="card">
      <CanisterStatus {canister} />
    </div>{/each}
</div>

<style lang="scss">
  .wrapper {
    display: flex;
    align-items: center;
    justify-content: left;
    flex-wrap: wrap;
  }

  .card {
    margin: 0 1rem;
    width: 28rem;
  }
</style>
