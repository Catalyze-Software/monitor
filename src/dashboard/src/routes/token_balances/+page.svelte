<script lang="ts">
  import { tokenBalances } from "$lib/api/monitor.api"
  import { Card } from "@dfinity/gix-components"
  import type { Principal } from "@dfinity/principal"
  import { onMount } from "svelte"

  let ready = false

  let balances: [Principal, bigint][] = []
  let totalPrincipals: bigint = 0n
  let totalPoints: bigint = 0n

  onMount(async () => {
    balances = await tokenBalances()
    totalPrincipals = BigInt(balances.length)
    balances.forEach((balance) => {
      totalPoints += balance[1]
    })

    ready = true
  })
</script>

<h1>Token canister point balances</h1>
{#if !ready}
  <p>Performing inter-canister call...</p>
{:else}
  <p>Total principals: {totalPrincipals}</p>
  <p>Total points: {totalPoints}</p>
  {#each balances as balance}
    <Card>
      <p>{balance[0]}</p>
      <p>Points: {balance[1]}</p>
    </Card>
  {/each}
{/if}
