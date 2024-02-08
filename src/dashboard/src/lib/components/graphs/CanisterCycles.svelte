<script lang="ts">
  import { cyclesToT } from "$lib/utils/tcycles.utils"
  import { onMount } from "svelte"
  import { Bar } from "svelte-chartjs"
  import type { ChartData } from "chart.js"

  import {
    Chart,
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale,
  } from "chart.js"
  import { sortedCanisterCycles } from "$lib/api/monitor.api"

  Chart.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

  let ready = false

  let data: ChartData<"bar", (number | [number, number])[], unknown> = {
    labels: [],
    datasets: [
      {
        label: "T Cycles",
        data: [],
        backgroundColor: "#794ee7",
        borderColor: "#794ee7",
        borderWidth: 1,
      },
    ],
  }

  onMount(async () => {
    const cycles = await sortedCanisterCycles()

    cycles.forEach((item) => {
      data.labels?.push(item.name)
      data.datasets[0].data.push(cyclesToT(item.cycles))
    })

    ready = true
  })
</script>

<h3>Cycle balances</h3>
{#if ready}
  <Bar {data} />
{:else}
  <p>Loading...</p>
{/if}
