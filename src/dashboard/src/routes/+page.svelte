<script lang="ts">
  import CanisterCycles from "$lib/components/graphs/CanisterCycles.svelte"
  import IcpHistory from "$lib/components/graphs/ICPHistory.svelte"
  import { Card } from "@dfinity/gix-components"
</script>

<div class="card-container">
  <div class="card">
    <Card>
      <IcpHistory />
    </Card>
  </div>

  <div class="card">
    <Card>
      <CanisterCycles />
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
