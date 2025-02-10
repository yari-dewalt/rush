<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { popupStore } from "$lib/state/popupStore.svelte";
  import { scale } from 'svelte/transition';

  let { process } = $props();
  
  async function killProcess() {
    await invoke('kill_process', { pid: process.pid });
    closePopup();
  }

  function closePopup() {
    popupStore.visible = false;
  }
</script>

<div
  class="relative rounded-lg flex flex-col gap-2 bg-primary w-96 overflow-hidden">
  <div class="flex flex-col text-center p-4 gap-4">
    <h3 class="text-lg text-text-primary font-medium">Are you sure you want to end the selected process "{process.name}" (PID: {process.pid})?</h3>
    <p class="text-sm text-text-secondary">Ending a process may destroy data, break the session or introduce a security risk. Only unresponsive processes should be ended.</p>
  </div>
  <div class="flex items-center w-full text-sm">
    <button class="hover:bg-background hover:text-text-primary w-1/2 p-3 text-text-secondary border-t border-r border-secondary font-medium" onclick={closePopup}>Cancel</button>
    <button class="hover:bg-background hover:text-red-500 w-1/2 p-3 text-red-400 border-t border-secondary font-medium" onclick={killProcess}>End Process</button>
  </div>
</div>
