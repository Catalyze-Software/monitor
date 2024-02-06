<script lang="ts">
  import { Card, Copy, KeyValuePair } from "@dfinity/gix-components"
  import type { CanisterCycles } from "$lib/declarations/monitor.did"
  import { cyclesToT } from "$lib/utils/tcycles.utils"

  export let canister: CanisterCycles
</script>

<Card>
  <h3 slot="start">{canister.name}</h3>
  <KeyValuePair>
    <span slot="key" class="key"
      >{canister.canister_id.toString()}
      <Copy value={canister.canister_id.toString()} /></span
    >
    <span slot="value"
      >{cyclesToT(canister.cycles)} <strong>TCycles</strong></span
    >
  </KeyValuePair>
</Card>

<style lang="scss">
  .key {
    display: flex;
    align-items: center;
  }
</style>
