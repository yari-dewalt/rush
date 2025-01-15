<script>
  import { invoke } from "@tauri-apps/api/core";
  import { createSettings } from '$lib/state/settings.svelte.ts';
  
  let prev = 0;
  let newInterval = $state('');
  const settings = createSettings();

  $effect(() => {
    const intervalId = setInterval(async () => {
      let info = await invoke('get_info');
      console.log(info);
      console.log(settings.interval);
    }, settings.interval);

    return () => {
      clearInterval(intervalId);
    }
  });
</script>

<h1>Test</h1>
<input class="border border-red-400" bind:value={newInterval}/>
<button onclick={() => {
  settings.interval = parseInt(newInterval);
  console.log("setting to", parseInt(newInterval))
  }}
>
  Update
</button>
