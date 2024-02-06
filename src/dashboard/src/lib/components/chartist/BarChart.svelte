<script lang="ts">
  import { onMount } from "svelte"
  import { BarChart, type BarChartData, type BarChartOptions } from "chartist"

  export let data: BarChartData
  export let options: BarChartOptions

  onMount(() => {
    new BarChart(".ct-chart", data, options)
  })
</script>

<div class="ct-chart"></div>
