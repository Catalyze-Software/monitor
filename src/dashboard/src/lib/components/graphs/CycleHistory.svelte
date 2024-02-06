<script lang="ts">
  import { convertTimestamp } from "$lib/utils/date.utils"
  import type { LineChartData, LineChartOptions } from "chartist"
  import { LineChart } from "chartist"
  import { onMount } from "svelte"
  import { cycleHistoryStore } from "$lib/stores/cycleHistory.store"

  let data: LineChartData = {
    labels: [],
    series: [],
  }

  let options: LineChartOptions = {
  }

  onMount(async () => {
    let labels: string[] = []
    let balances: number[][] = []

    $cycleHistoryStore.forEach((instant) => {
      labels.push(convertTimestamp(instant.timestamp))

      instant.balances.forEach((balance, index) => {
        if (balances[index] === undefined) {
          balances[index] = []
        }

        balances[index].push(balance[1])
      })
    })

    data.labels = labels
    data.series = balances

    new LineChart(".cycle-history-chart", data, options)
  })
</script>

<h3>Canister cycle history</h3>
<div class="cycle-history-chart"></div>
