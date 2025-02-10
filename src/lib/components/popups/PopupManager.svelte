<script lang="ts">
  import { popupStore } from '$lib/state/popupStore.svelte.ts';
  import ProcessInfoPopup from './ProcessInfoPopup.svelte';
  import KillProcessPopup from './KillProcessPopup.svelte';
	import SettingsPopup from './SettingsPopup.svelte';
  
  let { processes } = $props();
  let { visible, popup, content } = $derived(popupStore);

  function handleBackgroundClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      popupStore.visible = false;
    }
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="fixed inset-0 bg-black bg-opacity-30 z-50 flex items-center justify-center"
    onclick={handleBackgroundClick}
  >
    {#if popup === 'processInfo'}
      <ProcessInfoPopup {processes} process={content} /> 
    {/if}
    {#if popup == 'killProcess'}
      <KillProcessPopup process={content} />
    {/if}
    {#if popup === 'settings'}
      <SettingsPopup />
    {/if}
  </div>
{/if}
