<script lang="ts">
  import Panel from "./Panel.svelte";
  import { faMemory } from "@fortawesome/free-solid-svg-icons";
	import type { SystemInfo } from "$lib/types";
  import { convertSeconds } from '$lib/utils.ts';

  interface SystemPanelProps {
    systemInfo: SystemInfo,
    numProcesses: number,
  }

  let { systemInfo, numProcesses }: SystemPanelProps = $props();
</script>

<Panel icon={faMemory} title="System">
  <div class="flex flex-col text-xs gap-1.5">
    <div class="flex items-center justify-between gap-6">
      <p class="text-text-secondary">Name</p>
      <p class="text-text-primary whitespace-nowrap">{systemInfo.host_name}</p>
    </div>
    <div class="flex items-center justify-between gap-6">
      <p class="text-text-secondary">Uptime</p>
      <p class="text-text-primary whitespace-nowrap">{convertSeconds(systemInfo.uptime)}</p>
    </div>
    <div class="flex items-center justify-between gap-6">
      <p class="text-text-secondary whitespace-nowrap">Processes</p>
      <p class="text-text-primary whitespace-nowrap">{numProcesses}</p>
    </div>
  </div>
</Panel>
