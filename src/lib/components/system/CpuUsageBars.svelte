<script lang="ts">
  let { cpuUsages } = $props();
</script>

<div class="grid grid-cols-2 gap-x-4 gap-y-2 text-text-secondary max-h-28 overflow-y-auto">
  {#each cpuUsages as cpuUsage, i}
    <div class="flex justify-between items-center gap-3 w-3/4">
      <div class="flex items-center gap-1">
        <p class="text-xs whitespace-nowrap min-w-12">Core {i+1}</p>
        <div class="rounded h-2 grow min-w-14 w-3/4 bg-secondary overflow-hidden">
          <div
            class="h-2 bg-accent-primary"
            style="width: {cpuUsage}%">
          </div>
        </div>
      </div>
      <p class="text-xs font-bold min-w-10 text-right">{cpuUsage.toFixed(1)}%</p>
    </div>
  {/each}
</div>
