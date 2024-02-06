<script lang="ts">
  import { convertTimestamp } from "$lib/utils/date.utils"
  import { onMount } from "svelte"
  import { cycleHistoryStore } from "$lib/stores/cycleHistory.store"
  import { Line } from "svelte-chartjs"
  import type { ChartData, Point } from "chart.js"

  let ready = false

  let data: ChartData<"line", (number | Point)[], unknown> = {
    labels: [],
    datasets: [],
  }

  onMount(async () => {
    $cycleHistoryStore.forEach((instant, i) => {
      data.labels?.push(convertTimestamp(instant.timestamp))
      instant.balances.forEach((balance, index) => {
        if (i === 0) {
          data.datasets?.push({
            label: balance[0],
            data: [balance[1]],
          })
          return
        }
        data.datasets[index].data?.push(balance[1])
      })
    })

    ready = true
  })
</script>

<h3>Canister cycle history</h3>
{#if ready}
  <Line {data} />
{:else}
  <p>Loading...</p>
{/if}
