<script lang="ts">
  import CpuPanel from '$lib/components/system/CpuPanel.svelte';
  
  export let systemInfo;
</script>

<div class="flex p-2">
  <CpuPanel type='bars' cpuUsages={systemInfo.cpu_usage}/>
</div>
