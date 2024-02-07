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

  const colors = [
    "#794ee7", // Purple
    "#e38b1b", // Orange
    "#30af91", // Green
    "#198ae3", // Blue
    "#ea6c99", // Pink
    "#b871d9", // Orchid
    "#af63db", // Dark Orchid
    "#553b75", // Indigo
    "#3d4c7a", // Dark Blue
    "#f3ddbb", // Light Orange
    "#c7e3e1", // Light Green
    "#ebd4e7", // Light Pink
    "#c1d4f1", // Light Blue
    "#634a65", // Deep Violet
    "#3d2a5c", // Purple Dark
    "#2d1a4a", // Darker Purple
    "#291942", // Even Darker Purple
    "#24143d", // Almost Black Purple
    "#b080ff", // Light Purple
  ]

  onMount(async () => {
    $cycleHistoryStore.forEach((instant, i) => {
      data.labels?.push(convertTimestamp(instant.timestamp))
      instant.balances.forEach((balance, index) => {
        if (i === 0) {
          data.datasets?.push({
            label: balance[0],
            data: [balance[1]],
            borderColor: colors[index % colors.length],
            backgroundColor: "rgba(0, 0, 0, 0)",
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
