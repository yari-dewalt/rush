<script>
  import { createSettings } from '$lib/state/settings.svelte.ts';
  import { createSysProcessInfo }  from '$lib/state/sysProcessInfo.svelte.ts';
  import SystemStats from '$lib/components/system/SystemStats.svelte';
  
  let prev = 0;
  let newInterval = $state('');
  const settings = createSettings();
  const sysProcessInfo = createSysProcessInfo();

  const { processes, systemInfo } = $derived(sysProcessInfo);

  $effect(() => {
    const intervalId = setInterval(async () => {
      sysProcessInfo.getInfo();
      console.log(processes);
      console.log(systemInfo);
    }, settings.interval);

    return () => {
      clearInterval(intervalId);
    }
  });
</script>

<div class="dark bg-background flex flex-col min-h-screen w-full">
  <div class="max-h-1/4">
    <SystemStats {systemInfo} />
  </div>
  <div class="h-3/4 bg-secondary">
    <input class="border border-red-400" bind:value={newInterval}/>
    <button class="text-text-secondary" onclick={() => {
      settings.interval = parseInt(newInterval);
      console.log("setting to", parseInt(newInterval))
      }}
    >
      Update
    </button>
  </div>
</div>
