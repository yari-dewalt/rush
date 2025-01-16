<script lang="ts">
  let { cpuUsages } = $props();
</script>

<div class="grid grid-cols-2 gap-x-4 gap-y-1.5 overflow-y-auto">
  {#each cpuUsages as cpuUsage, i}
    <div class="flex justify-between items-center gap-1">
      <div class="flex items-center gap-1">
        <p class="text-xs whitespace-nowrap min-w-12 text-text-secondary">Core {i+1}</p>
        <div class="rounded h-2 grow min-w-14 w-3/4 bg-secondary overflow-hidden">
          <div
            class="h-2 bg-accent-primary"
            style="width: {cpuUsage}%">
          </div>
        </div>
      </div>
      <p class="text-xs font-bold min-w-8 text-right text-text-primary">{cpuUsage.toFixed(0)}%</p>
    </div>
  {/each}
</div>
