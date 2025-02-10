<script lang="ts">
	import { getContext } from "svelte";
  import { popupStore } from "$lib/state/popupStore.svelte";
  import { faXmark, faCircleInfo, faChevronDown } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
	import { Alert, Dropdown, DropdownDivider, DropdownItem } from "flowbite-svelte";
  import { fade } from 'svelte/transition';
	import type { Settings } from "$lib/types";

  const settings: Settings = getContext('settings');
  let showAlert = $state(false);
  let refreshDropDownOpen = $state(false);
  let themeDropDownOpen = $state(false);

  function closePopup() {
    popupStore.visible = false;
  }

  function saveSettings() {
    // Save settings
    showAlert = true;
    setTimeout(() => {
      showAlert = false;
    }, 3000);
  }

  function changeRefreshRate(newRate: number) {
    settings.interval = newRate;
    refreshDropDownOpen = false;
  }

  function changeTheme(newTheme: string) {
    settings.theme = newTheme;
    themeDropDownOpen = false;
  }
</script>

<div class="relative rounded-lg flex flex-col gap-2 bg-primary p-3 w-96 text-sm text-text-primary">
  <h2 class="text-base text-center">Settings</h2>
  <div class="min-h-[1px] w-full bg-secondary"></div>
  <button onclick={closePopup} class="absolute top-3 right-3 group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 hover:border-opacity-100">
    <Fa icon={faXmark} size="xs" class="text-text-secondary group-hover:text-text-primary" />
  </button>
  <div class="flex items-center justify-between">
    <h3>Refresh Rate</h3>
    <button id="refresh-dropdown" class="min-w-24 group rounded-md bg-secondary p-1.5 border border-text-secondary border-opacity-50 hover:border-opacity-100 text-text-secondary hover:text-text-primary text-xs pl-2 pr-2 flex items-center justify-between gap-1.5">
      <p class="whitespace-nowrap">{settings.interval}ms</p>
      <Fa icon={faChevronDown} class="text-text-secondary group-hover:text-text-primary" />
    </button>
    <Dropdown bind:open={refreshDropDownOpen} triggeredBy="#refresh-dropdown" class="min-w-24 border-[1px] border-text-secondary bg-secondary rounded-md text-text-secondary w-full">
      <DropdownItem onclick={() => changeRefreshRate(500)} class="p-2 text-xs hover:text-text-primary">500ms</DropdownItem>
      <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
      <DropdownItem onclick={() => changeRefreshRate(1000)} class="p-2 text-xs hover:text-text-primary">1000ms</DropdownItem>
      <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
      <DropdownItem onclick={() => changeRefreshRate(2000)} class="p-2 text-xs hover:text-text-primary">2000ms</DropdownItem>
    </Dropdown>
  </div>
  <div class="flex items-center justify-between">
    <h3>Theme</h3>
    <button id="theme-dropdown" class="min-w-24 group rounded-md bg-secondary p-1.5 border border-text-secondary border-opacity-50 hover:border-opacity-100 text-text-secondary hover:text-text-primary text-xs pl-2 pr-2 flex items-center justify-between gap-1.5">
      <p class="whitespace-nowrap">{settings.theme}</p>
      <Fa icon={faChevronDown} class="text-text-secondary group-hover:text-text-primary" />
    </button>
    <Dropdown bind:open={themeDropDownOpen} triggeredBy="#theme-dropdown" class="min-w-24 border-[1px] border-text-secondary bg-secondary rounded-md text-text-secondary w-full">
      <DropdownItem onclick={() => changeTheme('Light')} class="p-2 text-xs hover:text-text-primary">Light</DropdownItem>
      <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
      <DropdownItem onclick={() => changeTheme('Dark')} class="p-2 text-xs hover:text-text-primary">Dark</DropdownItem>
    </Dropdown>
  </div>

  <p>Show warning popup before killing task</p>
  <button onclick={saveSettings} class="w-fit ml-auto text-xs hover:border-opacity-100 hover:text-text-primary rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 text-text-secondary flex items-center justify-center">
    Save
  </button>
</div>

{#if showAlert}
  <div class="absolute bottom-4 right-4 text-text-secondary"
  transition:fade={{ duration: 200 }}>
    <Alert dismissable class="rounded-lg flex items-center gap-2 bg-primary p-3">
      <div slot="icon">
        <Fa icon={faCircleInfo} size="lg" class="text-green-300" />
      </div>
      <p class="text-green-300">Settings successfully saved!</p>

      <span class="sr-only"></span>
    </Alert>
  </div>
{/if}