<script lang="ts">
  import Panel from "./Panel.svelte";
  import CpuUsageBars from "./CpuUsageBars.svelte";

  import { faMicrochip } from "@fortawesome/free-solid-svg-icons";

  export let cpuUsages: Array<number>;
  export let type;
</script>

<Panel icon={faMicrochip} title="CPU Usage">
  {#if type === 'bars'}
    <CpuUsageBars {cpuUsages} />
  {/if}
  {#if type === 'pie'}
    <!-- <CpuUsagePie {cpuUsages} /> -->
  {/if}
  {#if type === 'graph'}
    <!-- <CpuUsageGraph {cpuUsages} /> -->
  {/if}
</Panel>
