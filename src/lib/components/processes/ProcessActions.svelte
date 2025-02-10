<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Fa from "svelte-fa";
  import { faThumbTack, faCircleInfo, faXmark } from "@fortawesome/free-solid-svg-icons";
  import { popupStore } from "$lib/state/popupStore.svelte";
  import { pinnedProcesses } from "$lib/state/pinnedProcesses.svelte";
  import { getContext } from "svelte";
  import type { Settings } from "$lib/types.ts";

  let { process } = $props();
  let isPinned = $state(false);
  let settings: Settings = getContext("settings");

  $effect(() => {
    isPinned = pinnedProcesses.has(process.pid);
  });

  function togglePin() {
    if (isPinned) {
      pinnedProcesses.delete(process.pid);
    } else {
      pinnedProcesses.add(process.pid);
    }
    isPinned = !isPinned;
  }

  function showProcessInfo() {
    popupStore.visible = true;
    popupStore.popup = 'processInfo';
    popupStore.content = process;
  }

  async function showKillProcessPopup() {
    popupStore.visible = true;
    popupStore.popup = 'killProcess';
    popupStore.content = process;
  }

  async function killProcess() {
    await invoke('kill_process', { pid: process.pid });
  }
</script>

<div class="flex items-center justify-center gap-2">
  <button onclick={togglePin} class="hover:opacity-100 text-accent-primary hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-accent-primary {isPinned ? 'bg-accent-primary text-text-primary opacity-100' : 'opacity-60'}">
    <Fa icon={faThumbTack} size="sm" />
  </button>
  <button onclick={showProcessInfo} class="opacity-60 hover:opacity-100 text-accent-primary hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-accent-primary">
    <Fa icon={faCircleInfo} size="sm" />
  </button>
  <button onclick={settings.showWarningPopup ? showKillProcessPopup : killProcess} class="opacity-60 hover:opacity-100 text-red-400 hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-red-400">
    <Fa icon={faXmark} size="sm" />
  </button>
</div>
