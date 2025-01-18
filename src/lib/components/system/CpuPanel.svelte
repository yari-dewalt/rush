<script lang="ts">
  import Panel from "./Panel.svelte";
  import CpuUsageBars from "./CpuUsageBars.svelte";

  import { faMicrochip } from "@fortawesome/free-solid-svg-icons";

  interface CpuPanelProps {
    cpuUsages: number[];
    type: string;
  };

  let { cpuUsages, type }: CpuPanelProps = $props();
  
  let usageValue = $derived(
    cpuUsages?.length ? cpuUsages.reduce((a, b) => a + b) / cpuUsages.length : 0
  );
</script>

<Panel icon={faMicrochip} title="CPU Usage" {usageValue}>
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
