<script lang="ts">
  import { icpBalance, latestIcpBalances } from "$lib/api/monitor.api"
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

  let balance = ""

  let data: ChartData<"line", (number | Point)[], unknown> = {
    labels: [],
    datasets: [
      {
        label: "ICP",
        data: [],
        borderColor: "#794ee7",
        backgroundColor: "rgba(121, 78, 231, 0.2)",
        borderWidth: 2,
        pointBackgroundColor: "#794ee7",
        pointBorderColor: "#fff",
        pointHoverBackgroundColor: "#fff",
        pointHoverBorderColor: "#794ee7",
      },
    ],
  }

  onMount(async () => {
    balance = await icpBalance()
    balance = balance.slice(0, 6)

    const n = 30n * (24n / 3n) // 30 days * 8 readings per day
    const response = await latestIcpBalances(n)

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
<p>Current balance: {balance} ICP</p>
{#if ready}
  <Line {data} />
{:else}
  <p>Loading...</p>
{/if}
