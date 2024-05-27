<script lang="ts">
  import { convertTimestamp } from "$lib/utils/date.utils"
  import { onMount } from "svelte"
  import { Line } from "svelte-chartjs"
  import type { ChartData, Point } from "chart.js"
  import {
    Chart,
    Title,
    Tooltip,
    Legend,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
  } from "chart.js"

  // Backend function that returns a cycles balance history for every canister
  import { canisterCycleHistory } from "$lib/api/monitor.api"

  let ready = false

  Chart.register(
    Title,
    Tooltip,
    Legend,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement
  )

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
    const n = 30n * (24n / 3n) // 30 days * 8 readings per day
    const cycleHistory = await canisterCycleHistory(n)

    cycleHistory.timestamps.forEach((timestamp) => {
      data.labels?.push(convertTimestamp(timestamp))
    })

    cycleHistory.line_data.forEach((line_data, index) => {
      data.datasets?.push({
        label: line_data.canister_name,
        data: line_data.cycles,
        borderColor: colors[index % colors.length],
        backgroundColor: "rgba(0, 0, 0, 0)",
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
