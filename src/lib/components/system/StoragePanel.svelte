<script lang="ts">
  import Panel from "./Panel.svelte";
  import StorageUsageText from "./StorageUsageText.svelte";
  import DiskList from "./DiskList.svelte";

  import { convertBytes } from '$lib/utils.ts';
  import { faServer} from "@fortawesome/free-solid-svg-icons";
	import type { Disk, StorageData } from "$lib/types";

  interface StoragePanelProps {
    storageData: StorageData,
    disks: Disk[],
    type: string,
  }

  let { storageData, disks, type }: StoragePanelProps = $props();
</script>

<Panel icon={faServer} title="Storage" usageValue={(storageData.used / storageData.total) * 100} hoverInfo={`${convertBytes(storageData.used)} / ${convertBytes(storageData.total)}`}>
  <div class="overflow-y-auto flex flex-col gap-1.5">
    {#if type === 'text'}
      <StorageUsageText {storageData} />
    {/if}
    {#if type === 'graph'}
      <!-- <DiskGraph {diskData} /> -->
    {/if}
    <div class="min-h-[1px] w-full bg-secondary"></div>
    <DiskList {disks} />
  </div>
</Panel>
