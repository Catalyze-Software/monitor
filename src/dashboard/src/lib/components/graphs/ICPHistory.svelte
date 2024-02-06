<script lang="ts">
  import { latestIcpBalances } from "$lib/api/monitor.api"
  import { convertTimestamp } from "$lib/utils/date.utils"
  import { onMount } from "svelte"
  import type { ChartData, Point } from "chart.js"

  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
  } from "chart.js"
  import { Line } from "svelte-chartjs"

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
  )

  let ready = false

  let data: ChartData<"line", (number | Point)[], unknown> = {
    labels: [],
    datasets: [
      {
        label: "ICP",
        data: [],
      },
    ],
  }

  onMount(async () => {
    const response = await latestIcpBalances(1000n)

    let labels: string[] = []
    let balances: number[] = []

    response.forEach((item) => {
      labels.push(convertTimestamp(item[0]))
      balances.push(item[1])
    })

    data.labels = labels

    data.datasets[0].data = balances

    ready = true
  })
</script>

<h3>Monitor ICP balance history</h3>
{#if ready}
  <Line {data} />
{:else}
  <p>Loading...</p>
{/if}
