<script lang="ts">
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

  import { sortedMemorySizes } from "$lib/api/monitor.api"
  import type { CanisterMemorySize } from "$lib/declarations/monitor.did"

  Chart.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

  let ready = false

  let data: ChartData<"bar", (number | [number, number])[], unknown> = {
    labels: [],
    datasets: [
      {
        label: "Mb",
        data: [],
        backgroundColor: "#794ee7",
        borderColor: "#794ee7",
        borderWidth: 1,
      },
    ],
  }

  onMount(async () => {
    const memorySizes: CanisterMemorySize[] = await sortedMemorySizes()

    memorySizes.forEach((item) => {
      data.labels?.push(item.name)
      data.datasets[0].data.push(Number(item.size / 1024n / 1024n))
    })

    ready = true
  })
</script>

<h3>Canister memory size</h3>
{#if ready}
  <Bar {data} />
{:else}
  <p>Loading...</p>
{/if}
