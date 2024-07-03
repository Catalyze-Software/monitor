<script lang="ts">
  import MemberActivityRewards from "$lib/components/graphs/MemberActivityRewards.svelte";
  import MemberCountRewards from "$lib/components/graphs/MemberCountRewards.svelte";
  import { Card } from "@dfinity/gix-components";
</script>

<div class="card-container">
  <div class="card">
    <Card>
      <MemberCountRewards />
    </Card>
  </div>
  <div class="card">
    <Card>
      <MemberActivityRewards />
    </Card>
  </div>
</div>

<style>
  .card-container {
    display: flex;
    justify-content: left;
    flex-wrap: wrap;
  }

  .card {
    margin: 0 1rem;
    min-width: 250px;
    max-width: 600px;
    width: 100%;
  }
</style>
