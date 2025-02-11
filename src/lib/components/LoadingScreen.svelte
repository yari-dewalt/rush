<script lang="ts">
  import { fade } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
</script>

<div
  class="fixed inset-0 z-50 flex flex-col gap-2 items-center justify-center bg-[#23232B]"
  transition:fade={{ duration: 300, easing: quintOut }}>
  <img class="w-32 h-32" src="/appIcon.png" alt="Rush App Icon"/>
  <h1 class="text-white font-bold text-3xl">Rush</h1>
</div>