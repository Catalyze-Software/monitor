<script lang="ts">
  import { latestIcpBalances } from "$lib/api/monitor.api"
  import LineChart from "$lib/components/graphs/LineChart.svelte"
  import type { LineChartData, LineChartOptions } from "chartist"
  import { onMount } from "svelte"

  let ready = false

  let data: LineChartData = {
    labels: [],
    series: [],
  }

  let options: LineChartOptions = {
    fullWidth: true,
    chartPadding: {
      right: 40,
    },
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
    data.series = [balances, balances]

    console.log(data)

    ready = true
  })

  function convertTimestamp(timestamp: bigint): string {
    const date = new Date(Number(timestamp) / 1000)
    return date.toLocaleDateString()
  }
</script>

{#if ready}
<LineChart {data} {options} />
{/if}

<style lang="scss" global>
  @import "../../node_modules/chartist/dist/index.scss";
  @import "../../node_modules/chartist/dist/_settings.scss";
</style>
