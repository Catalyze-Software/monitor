<script lang="ts">
  import { canisterStore } from "$lib/stores/canisters.store"
  import { cyclesToT } from "$lib/utils/tcycles.utils"
  import { BarChart, type BarChartData, type BarChartOptions } from "chartist"
  import { onMount } from "svelte"

  let data: BarChartData = {
    labels: [],
    series: [],
  }

  let options: BarChartOptions = {
    horizontalBars: true,
    height: 500,
    reverseData: true,
  }

  onMount(async () => {
    let labels: string[] = []
    let balances: number[] = []

    $canisterStore.forEach((item) => {
      labels.push(item.name)
      balances.push(cyclesToT(item.cycles))
    })

    data.labels = labels
    data.series = [balances]

    new BarChart(".cycle-balances-chart", data, options)
  })
</script>

<h3>Cycle balances</h3>
<div class="cycle-balances-chart"></div>
