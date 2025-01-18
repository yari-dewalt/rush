<script lang="ts">
  import Fa from "svelte-fa";
  import { Popover } from 'flowbite-svelte';
  import { faHardDrive } from "@fortawesome/free-solid-svg-icons";
  import { convertBytes } from '$lib/utils.ts';
	import type { Disk } from "$lib/types";

  let { disks }: { disks: Disk[] } = $props();
</script>

<div class="flex flex-col gap-1.5">
  {#each disks as disk, i}
    <div id="disk-{i}" class="group flex items-center gap-2">
      <Fa icon={faHardDrive} size="xs" class="text-text-secondary group-hover:text-text-primary"/>
      <p class="text-xs text-text-secondary group-hover:text-text-primary">{disk.name}</p>
      <div class="rounded h-2 grow min-w-14 w-3/4 bg-secondary overflow-hidden">
        <div
          class="h-2 bg-accent-primary"
          style="width: {((disk.totalSpace - disk.availableSpace) / disk.totalSpace) * 100}%">
        </div>
      </div>
      <p class="text-xs font-bold min-w-8 text-right text-text-primary">{(((disk.totalSpace - disk.availableSpace) / disk.totalSpace) * 100).toFixed(0)}%</p>
    </div>
    <Popover triggeredBy="#disk-{i}" placement="bottom">
      <div class="bg-primary outline outline-2 outline-secondary text-xs text-text-primary flex flex-col items-center justify-center p-2">
        <h3>{convertBytes(disk.totalSpace - disk.availableSpace, 0)} / {convertBytes(disk.totalSpace, 0)}</h3>
        <p>R: {convertBytes(disk.readUsage, 0)}</p>
        <p>W: {convertBytes(disk.writeUsage, 0)}</p>
      </div>
    </Popover>
  {/each}
</div>
