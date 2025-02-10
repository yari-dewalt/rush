<script lang="ts">
  import { faCircleInfo, faFolder, faXmark } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { popupStore } from "$lib/state/popupStore.svelte";
  import { Command } from '@tauri-apps/plugin-shell';

  let { process, x, y } = $props();

  function showProcessInfo() {
    popupStore.visible = true;
    popupStore.popup = 'processInfo';
    popupStore.content = process;
  }

  async function openProcessLocation() {
    try {
      // Using xdg-open for Linux
      const dirPath = process.exe.split('/').slice(0, -1).join('/');
      const command = await Command.create('xdg-open', [dirPath]);
      await command.execute();
    } catch (error) {
      console.error('Failed to open process location:', error);
    }
  }

  async function showKillProcessPopup() {
    popupStore.visible = true;
    popupStore.popup = 'killProcess';
    popupStore.content = process;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  class="absolute bg-primary outline outline-2 outline-secondary text-xs text-text-primary flex flex-col justify-center" 
  oncontextmenu={(e) => e.preventDefault()}
  style="left: {x}px; top: {y}px;"
>
  <button onclick={showProcessInfo} class="flex items-center gap-2 p-2 opacity-80 hover:opacity-100 hover:bg-background">
    <Fa icon={faCircleInfo} />
    <p>Properties</p>
  </button>
  <div class="h-[2px] bg-secondary"></div>
  <button disabled={!process.exe} onclick={openProcessLocation} class="{!process.exe ? 'opacity-30' : 'opacity-80 hover:opacity-100 hover:bg-background'} flex items-center gap-2 p-2">
    <Fa icon={faFolder} />
    <p>Open location</p>
  </button>
  <div class="h-[2px] bg-secondary"></div>
  <button onclick={showKillProcessPopup} class="flex items-center gap-2 p-2 opacity-80 hover:opacity-100 hover:bg-background">
    <Fa icon={faXmark} color="#ff6467" />
    <p>End process</p>
  </button>
</div>
