<script lang="ts">
	import type { MemoryData } from '$lib/types';
  import { convertBytes } from '$lib/utils.ts';
  
  let { memoryData }: { memoryData: MemoryData } = $props();
</script>

<div class="flex flex-col text-xs gap-1.5">
  <div class="flex items-center justify-between gap-4">
    <p class="text-text-secondary whitespace-nowrap">Memory usage</p>
    <div class="rounded h-2 grow min-w-20 bg-secondary overflow-hidden">
      <div
        class="h-2 bg-accent-primary"
        style="width: {(memoryData.usedMemory / memoryData.totalMemory) * 100}%">
      </div>
    </div>
    <p class="text-text-primary">{((memoryData.usedMemory / memoryData.totalMemory) * 100).toFixed(0)}%</p>
  </div>
  <div class="flex items-center justify-between">
    <p class="text-text-secondary">Total</p>
    <p class="text-text-primary">{convertBytes(memoryData.totalMemory, 2)}</p>
  </div>
  <div class="flex items-center justify-between">
    <p class="text-text-secondary">Used</p>
    <p class="text-text-primary">{convertBytes(memoryData.usedMemory, 2)}</p>
  </div>
  <div class="flex items-center justify-between">
    <p class="text-text-secondary">Free</p>
    <p class="text-text-primary">{convertBytes(memoryData.freeMemory, 2)}</p>
  </div>
</div>
