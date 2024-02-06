<script lang="ts">
  import { onMount } from "svelte"
  import { LineChart } from "chartist"
  import type { LineChartData, LineChartOptions } from "chartist"

  export let data: LineChartData
  export let options: LineChartOptions

  onMount(() => {
    new LineChart(".ct-chart", data, options)
  })
</script>

<div class="ct-chart"></div>
