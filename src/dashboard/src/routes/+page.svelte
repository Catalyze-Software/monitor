<script lang="ts">
  import CanisterCycles from "$lib/components/graphs/CanisterCycles.svelte"
  import CycleHistory from "$lib/components/graphs/CycleHistory.svelte"
  import IcpHistory from "$lib/components/graphs/ICPHistory.svelte"
  import { Card } from "@dfinity/gix-components"
</script>

<Card>
  <IcpHistory />
</Card>

<Card>
  <CanisterCycles />
</Card>

<Card>
  <CycleHistory />
</Card>

<style lang="scss" global>
  @import "../../node_modules/chartist/dist/index.scss";
  @import "../../node_modules/chartist/dist/_settings.scss";
</style>
