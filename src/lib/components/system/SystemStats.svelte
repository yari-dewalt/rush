<script lang="ts">
  import CpuPanel from '$lib/components/system/CpuPanel.svelte';
  import MemoryPanel from '$lib/components/system/MemoryPanel.svelte';
  import StoragePanel from '$lib/components/system/StoragePanel.svelte';
	import type { MemoryData, StorageData, SystemInfo } from '$lib/types';
  
  let { systemInfo }: { systemInfo: SystemInfo } = $props();

  let cpuUsages: number[] = $derived(systemInfo.cpu_usage);

  let memoryData: MemoryData = $derived({
    totalMemory: systemInfo.total_memory,
    freeMemory: systemInfo.free_memory,
    usedMemory: systemInfo.used_memory
  });

  let storageData: StorageData = $derived({
    total: systemInfo.disk_total_bytes,
    free: systemInfo.disk_free_bytes,
    used: systemInfo.disk_used_bytes,
    diskRead: systemInfo.disk_read,
    diskWrite: systemInfo.disk_write,
  });
  
  let disks: Disk[] = $derived(systemInfo.disks?.map(disk => ({
    name: disk.name,
    totalSpace: disk.total_space,
    availableSpace: disk.available_space,
    readUsage: disk.usage[0],
    writeUsage: disk.usage[1],
  })));
</script>

<div class="flex p-2 gap-2 h-[10.5rem]">
  <CpuPanel type='bars' {cpuUsages} />
  <MemoryPanel type='bars' {memoryData} />
  <StoragePanel type='text' {storageData} {disks} />
</div>
