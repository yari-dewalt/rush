<script lang="ts">
  import type { Process } from '$lib/types.ts';
  import ProcessIcon from './ProcessIcon.svelte';
  import { convertBytes, convertSeconds } from '$lib/utils.ts';
  import ProcessActions from './ProcessActions.svelte';

  let { process, onContextMenu }: { process: Process, onContextMenu: (e: MouseEvent) => void } = $props();

  function handleContextMenu(e: MouseEvent) {
    if (!(e.target as HTMLElement).closest('button')) {
      onContextMenu(e);
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div oncontextmenu={handleContextMenu} class="flex items-center w-full bg-background hover:bg-secondary p-2 text-sm border-t border-b border-secondary text-text-secondary">
  <div class="flex items-center gap-2 min-w-[11.1%] max-w-[11.1%]">
    <ProcessIcon processName={process.name} />
    <p class="text-text-primary truncate">{process.name}</p>
  </div>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{process.pid}</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{process.status}</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{process.cpu_usage.toFixed(1)}%</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{convertBytes(process.memory_usage)}</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{convertBytes(process.virtual_memory)}</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{convertBytes(process.disk_usage[0])} / {convertBytes(process.disk_usage[1])}</p>
  <p class="min-w-[11.1%] max-w-[11.1%] whitespace-nowrap">{convertSeconds(process.run_time)}</p>
  <div class="min-w-[11.1%] max-w-[11.1%]">
    <ProcessActions {process} />
  </div>
</div>
