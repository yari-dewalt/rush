<script lang="ts">
  import Fa from "svelte-fa";
  import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
  import type { Snippet } from 'svelte';
  import { Popover } from 'flowbite-svelte';

  interface PanelProps {
    children: Snippet;
    icon: IconDefinition;
    title: string;
    usageValue?: number;
    hoverInfo?: string;
  }

  let { children, icon, title, usageValue, hoverInfo }: PanelProps = $props();
</script>

<div class="rounded-lg flex flex-col gap-2 bg-primary p-3 max-h-full grow min-w-fit">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2">
      <Fa {icon} class="text-text-primary"/>
      <h2 class="text-sm text-text-primary">{title}</h2>
    </div>
    {#if usageValue}
      <div id="{title}-usagevalue" class="text-xs text-text-primary font-bold rounded-xl bg-secondary pl-2 pr-2 p-0.5">
        {usageValue.toFixed(1)}%
      </div>
    {/if}
  </div>
  <Popover triggeredBy="#{title}-usagevalue" placement="bottom" class={hoverInfo ? '' : 'opacity-0'}>
    {#if hoverInfo}
      <div class="bg-primary outline outline-2 outline-secondary text-xs text-text-primary flex items-center justify-center p-2">
        <h3>{hoverInfo}</h3>
      </div>
    {/if}
  </Popover>
  <div class="min-h-[1px] w-full bg-secondary"></div>
  {@render children?.()}
</div>
