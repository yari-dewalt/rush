<script lang="ts">
  import Panel from "./Panel.svelte";
  import MemoryBars from "./MemoryBars.svelte";

  import { faMemory } from "@fortawesome/free-solid-svg-icons";
	import type { MemoryData } from "$lib/types";

  interface MemoryPanelProps {
    memoryData: MemoryData;
    type: string;
  };

  let { memoryData, type }: MemoryPanelProps = $props();
  let usageValue = $derived((memoryData.usedMemory / memoryData.totalMemory) * 100);
</script>

<Panel icon={faMemory} title="Memory" {usageValue}>
  {#if type === 'bars'}
    <MemoryBars {memoryData} />
  {/if}
  {#if type === 'pie'}
    <!-- <MemoryPie {memory} /> -->
  {/if}
  {#if type === 'graph'}
    <!-- <CpuUsageGraph {cpuUsages} /> -->
  {/if}
</Panel>
