<script lang="ts">
  import CpuPanel from '$lib/components/system/CpuPanel.svelte';
  import MemoryPanel from '$lib/components/system/MemoryPanel.svelte';
  
  let { systemInfo } = $props();

  let cpuUsages = $derived(systemInfo.cpu_usage);
  let memoryData = $derived({
    total_memory: systemInfo.total_memory,
    free_memory: systemInfo.free_memory,
    used_memory: systemInfo.used_memory
  });
</script>

<div class="flex p-2 gap-2 h-40">
  <CpuPanel type='bars' {cpuUsages} />
  <MemoryPanel type='bars' {memoryData} />
</div>
