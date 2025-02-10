<script lang="ts">
  import Fa from "svelte-fa";
  import { faThumbTack, faCircleInfo, faXmark } from "@fortawesome/free-solid-svg-icons";
  import { popupStore } from "$lib/state/popupStore.svelte";
  import { pinnedProcesses } from "$lib/state/pinnedProcesses.svelte";

  let { process } = $props();
  let isPinned = $state(false);

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
</script>

<div class="flex items-center justify-center gap-2">
  <button onclick={togglePin} class="hover:opacity-100 text-accent-primary hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-accent-primary {isPinned ? 'bg-accent-primary text-text-primary opacity-100' : 'opacity-60'}">
    <Fa icon={faThumbTack} size="sm" />
  </button>
  <button onclick={showProcessInfo} class="opacity-60 hover:opacity-100 text-accent-primary hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-accent-primary">
    <Fa icon={faCircleInfo} size="sm" />
  </button>
  <button onclick={showKillProcessPopup} class="opacity-60 hover:opacity-100 text-red-400 hover:text-text-primary min-w-7 min-h-7 flex items-center justify-center rounded-md border-2 border-red-400">
    <Fa icon={faXmark} size="sm" />
  </button>
</div>
