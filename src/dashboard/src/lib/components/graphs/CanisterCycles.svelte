<script lang="ts">
  import { canisterStore } from "$lib/stores/canisters.store"
  import BarChart from "$lib/components/chartist/BarChart.svelte"
  import { cyclesToT } from "$lib/utils/tcycles.utils"
  import type { BarChartData, BarChartOptions } from "chartist"
  import { onMount } from "svelte"

  let ready = false

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

    ready = true
  })
</script>

{#if ready}
  <h3>Cycle balances</h3>
  <BarChart {data} {options} />
{/if}
