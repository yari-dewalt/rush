<script lang="ts">
	import type { StorageData } from '$lib/types';
  import { convertBytes } from '$lib/utils.ts';

  let { storageData }: { storageData: StorageData } = $props();
</script>

<div class="flex flex-col text-xs gap-1.5">
  <div class="flex items-center justify-between">
    <p class="text-text-secondary">Read</p>
    <p class="text-text-primary">{convertBytes(storageData.diskRead, 0)}</p>
  </div>
  <div class="flex items-center justify-between">
    <p class="text-text-secondary">Write</p>
    <p class="text-text-primary">{convertBytes(storageData.diskWrite, 0)}</p>
  </div>
</div>
