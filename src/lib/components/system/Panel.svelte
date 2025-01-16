<script lang="ts">
  import Fa from "svelte-fa";
  import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";

  let { icon, title, usageValue } = $props();
</script>

<div class="rounded-lg flex flex-col gap-2 bg-primary p-3">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2">
      <Fa {icon} class="text-text-primary"/>
      <h2 class="text-sm text-text-primary">{title}</h2>
    </div>
    <div class="text-xs text-text-primary font-bold rounded-xl bg-secondary pl-2 pr-2 p-0.5">
      {usageValue.toFixed(1)}%
    </div>
  </div>
  <div class="h-[1px] w-full bg-secondary"></div>
  <slot></slot>
</div>
