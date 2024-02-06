<script lang="ts">
  import { latestIcpBalances } from "$lib/api/monitor.api"
  import { convertTimestamp } from "$lib/utils/date.utils"
  import { LineChart, type LineChartData, type LineChartOptions } from "chartist"
  import { onMount } from "svelte"

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
    // single balance array doesnt render graph?
    data.series = [balances, balances]

    new LineChart(".icp-history-chart", data, options)
  })
</script>

<h3>Monitor ICP balance history</h3>
<div class="icp-history-chart"></div>
