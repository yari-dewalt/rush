<script lang="ts">
  import Fa from "svelte-fa";
  import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";

  let { icon, title, usageValue } = $props();
</script>

<div class="rounded-lg flex flex-col gap-1 bg-primary p-3">
  <div class="flex justify-between items-center">
    <div class="flex items-center gap-2">
      <Fa {icon} class="text-text-primary"/>
      <h2 class="text-sm text-text-primary">{title}</h2>
    </div>
    <div class="rounded bg-secondary">
      {usageValue}
    </div>
  </div>
  <div class="h-[1px] w-full bg-secondary"></div>
  <slot></slot>
</div>
