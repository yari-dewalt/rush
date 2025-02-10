<script lang="ts">
	import { convertBytes, convertSeconds, formatEpochTime } from "$lib/utils";
  import { faXmark } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import ProcessIcon from '../processes/ProcessIcon.svelte';
  import { popupStore } from "$lib/state/popupStore.svelte";

  let { processes, process } = $props();

  // Get updated process info
  let currentProcess = $derived(
    processes.find((p) => p.pid === process.pid)
  );

  function closePopup() {
    popupStore.visible = false;
  }
</script>

<div class="relative rounded-lg flex flex-col gap-2 bg-primary p-3 w-96">
  <button onclick={closePopup} class="absolute top-3 right-3 group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 hover:border-opacity-100">
    <Fa icon={faXmark} size="xs" class="text-text-secondary group-hover:text-text-primary" />
  </button>
  <div class="flex gap-2 justify-center items-center">
    <ProcessIcon processName={process.name} />
    <p class="text-text-primary font-medium text-center">{currentProcess.name} (PID {currentProcess.pid})</p>
  </div>
  <div class="min-h-[1px] w-full bg-secondary"></div>
  <div class="flex flex-col gap-1.5 text-sm p-2 rounded bg-secondary whitespace-nowrap">
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Process Name</p>
      <p class="text-text-secondary">{currentProcess.name ?? '-'}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">PID</p>
      <p class="text-text-secondary">{currentProcess.pid}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Parent PID</p>
      <p class="text-text-secondary">{currentProcess.ppid}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">User ID</p>
      <p class="text-text-secondary">{currentProcess.user_id}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Status</p>
      <p class="text-text-secondary">{currentProcess.status}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">CPU %</p>
      <p class="text-text-secondary">{currentProcess.cpu_usage.toFixed(2)}%</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Memory</p>
      <p class="text-text-secondary">{convertBytes(currentProcess.memory_usage, 2)}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Virtual Memory</p>
      <p class="text-text-secondary">{convertBytes(currentProcess.virtual_memory, 2)}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Disk Read</p>
      <p class="text-text-secondary">{convertBytes(currentProcess.disk_usage[0], 2)}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Disk Write</p>
      <p class="text-text-secondary">{convertBytes(currentProcess.disk_usage[1], 2)}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Command Line</p>
      <p class="text-text-secondary">{currentProcess.cmd || '-'}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Path</p>
      <p class="text-text-secondary">{currentProcess.exe || '-'}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Started</p>
      <p class="text-text-secondary">{formatEpochTime(currentProcess.start_time)}</p>
    </div>
    <div class="flex justify-between items-center gap-6 overflow-x-auto">
      <p class="text-text-primary">Total Runtime</p>
      <p class="text-text-secondary">{convertSeconds(currentProcess.run_time)}</p>
    </div>
  </div>
</div>
