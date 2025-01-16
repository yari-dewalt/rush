<script lang="ts">
  import Panel from "./Panel.svelte";
  import MemoryBars from "./MemoryBars.svelte";

  import { faMemory } from "@fortawesome/free-solid-svg-icons";

  let { memoryData, type } = $props();
  let usageValue = $derived((memoryData.used_memory / memoryData.total_memory) * 100);
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
