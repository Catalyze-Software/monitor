<script lang="ts">
  import { canisterStore } from "$lib/stores/canisters.store"
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

  Chart.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

  let ready = false

  let data: ChartData<"bar", (number | [number, number])[], unknown> = {
    labels: [],
    datasets: [
      {
        label: "T Cycles",
        data: [],
      },
    ],
  }

  onMount(async () => {
    $canisterStore.forEach((item) => {
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
