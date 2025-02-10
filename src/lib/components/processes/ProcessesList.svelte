<script lang="ts">
  import type { Process } from '$lib/types.ts';
  import ProcessComponent from './Process.svelte';
  import ProcessContextMenu from './ProcessContextMenu.svelte';
  import { onMount, onDestroy } from 'svelte';

  let { processes }: { processes: Process[] } = $props();
  let menuRef: HTMLDivElement | undefined = $state(undefined);

  let menuDimensions;
  $effect(() => {
    if (menuRef) {
      menuDimensions = menuRef.getBoundingClientRect();
      console.log(menuDimensions);
    }
  })

  let contextMenu = $state({
    visible: false,
    x: 0,
    y: 0,
    process: null as Process | null
  });

  function handleClickOutside(event: MouseEvent) {
    if (contextMenu.visible && menuRef && !menuRef.contains(event.target as Node)) {
      hideContextMenu();
    }
  }

  onMount(() => {
    if (typeof document !== 'undefined') {
      document.addEventListener('click', handleClickOutside);
    }
  });

  onDestroy(() => {
    if (typeof document !== 'undefined') {
      document.removeEventListener('click', handleClickOutside);
    }
  });

  function updateContextMenu(process: Process, event: MouseEvent) {
    event.preventDefault();
    event.stopPropagation();

    const windowScroll = window.scrollY;

    contextMenu = {
      visible: true,
      x: event.clientX,
      y: event.clientY + windowScroll,
      process
    };
  }

  function hideContextMenu() {
    contextMenu.visible = false;
  }
</script>

<div class="flex flex-col">
  {#each processes as process}
    <ProcessComponent {process} onContextMenu={(e: MouseEvent) => updateContextMenu(process, e)} />
  {/each}
</div>

{#if contextMenu.visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div onclick={hideContextMenu} bind:this={menuRef}>
    <ProcessContextMenu process={contextMenu.process} x={contextMenu.x} y={contextMenu.y} />
  </div>
{/if}

