<script>
  import { createSettings } from '$lib/state/settings.svelte.ts';
  import { createSysProcessInfo }  from '$lib/state/sysProcessInfo.svelte.ts';
  import SystemStats from '$lib/components/system/SystemStats.svelte';
  import Processes from '$lib/components/processes/Processes.svelte';
  import PopupManager from '$lib/components/popups/PopupManager.svelte';
  import { setContext } from 'svelte';
  
  const settings = createSettings();
  const sysProcessInfo = createSysProcessInfo();
  setContext('settings', settings);

  let { processes, systemInfo } = $derived(sysProcessInfo);
  let numProcesses = $derived(processes.length);

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

<div class="{settings.theme.toLowerCase()} bg-background flex flex-col min-h-screen max-h-screen w-full overflow-x-hidden">
  <div class="h-1/4">
    <SystemStats {systemInfo} {numProcesses} />
  </div>
  <div class="h-3/4 overflow-y-auto">
    <Processes {processes} />
  </div>
  <PopupManager {processes} />
</div>
