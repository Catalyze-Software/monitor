<script lang="ts">
  import { Card, Copy } from "@dfinity/gix-components"
  import type { Principal } from "@dfinity/principal"

  export let principal: string
</script>

<Card>
  <h3 slot="start">You are not registered!</h3>
  <div>Your principal</div>

  <div class="principal">
    <strong>{principal} </strong><Copy value={principal} />
  </div>
</Card>

<style lang="scss">
  .principal {
    display: flex;
    align-items: center;
    justify-content: left;
  }
</style>
