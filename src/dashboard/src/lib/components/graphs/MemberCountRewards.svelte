<script lang="ts">
  import { graphMemberCountRewards } from "$lib/api/monitor.api"
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
        label: "Points",
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

  let graph: [bigint, bigint][] = []

  onMount(async () => {
    graph = await graphMemberCountRewards()

    graph.forEach((item) => {
      data.labels?.push(item[0])
      data.datasets[0].data.push(Number(item[1]))
    })

    ready = true
  })
</script>

<h3>Member count rewards</h3>
{#if ready}
  <Line {data} />

  <table>
    <thead>
      <tr>
        <th>Milestone</th>
        <th>Member count</th>
        <th>Points</th>
      </tr>
    </thead>
    <tbody>
      {#each graph as dataPoint, i}
        <tr>
          <td>{i}</td>
          <td>{dataPoint[0]}</td>
          <td>{dataPoint[1]}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{:else}
  <p>Loading...</p>
{/if}

<style>
  table {
    margin-top: 1rem;
  }

  th {
    text-align: left;
  }
</style>
