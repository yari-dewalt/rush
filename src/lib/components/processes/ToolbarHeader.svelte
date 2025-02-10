<script lang="ts">
  import { faUpDown, faArrowUp, faArrowDown, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  type ArrowState = 'upDown' | 'asc' | 'desc';

  let { text, arrowState, clickHandler } = $props<{
    text: string;
    arrowState: ArrowState;
    clickHandler: () => void;
  }>();

  const stateToIcon: Record<ArrowState, IconDefinition> = {
    upDown: faUpDown,
    asc: faArrowUp,
    desc: faArrowDown
  };

  let icon = $derived(stateToIcon[arrowState]);
</script>

<button onclick={clickHandler} class="flex items-center gap-1 group hover:cursor-pointer {arrowState === 'upDown' ? 'hover:opacity-85' : ''}">
  <p class="group-hover:text-text-primary whitespace-nowrap {arrowState !== 'upDown' ? 'text-text-primary' : ''}">{text}</p>
  <Fa {icon} size="sm" class="group-hover:text-text-primary {arrowState !== 'upDown' ? 'text-text-primary' : 'text-text-secondary'}"/>
</button>
