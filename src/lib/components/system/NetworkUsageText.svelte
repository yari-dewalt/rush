<script lang="ts">
	import type { NetworkData } from '$lib/types';
  import { convertBytes } from '$lib/utils.ts';

  let { networkData }: { storageData: NetworkData } = $props();
</script>

<div class="flex flex-col text-xs gap-1.5">
  <div class="flex items-center justify-between gap-6">
    <p class="text-text-secondary">Receiving</p>
    <p class="text-text-primary whitespace-nowrap">{convertBytes(networkData.received, 0)}</p>
  </div>
  <div class="flex items-center justify-between gap-6">
    <p class="text-text-secondary">Sending</p>
    <p class="text-text-primary whitespace-nowrap">{convertBytes(networkData.transmitted, 0)}</p>
  </div>
  <div class="flex items-center justify-between gap-6">
    <p class="text-text-secondary whitespace-nowrap">Total Received</p>
    <p class="text-text-primary whitespace-nowrap">{convertBytes(networkData.totalReceived, 0)}</p>
  </div>
  <div class="flex items-center justify-between gap-6">
    <p class="text-text-secondary whitespace-nowrap">Total Sent</p>
    <p class="text-text-primary whitespace-nowrap">{convertBytes(networkData.totalTransmitted, 0)}</p>
  </div>
</div>
