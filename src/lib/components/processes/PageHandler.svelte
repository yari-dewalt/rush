<script lang="ts">
  import { faChevronDown } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Dropdown, DropdownItem, DropdownDivider } from 'flowbite-svelte';
  import { getContext } from 'svelte';
	import type { Settings } from "$lib/types";

  let { processes, pageNumber, setPageNumber } = $props();

  const settings: Settings = getContext('settings');
  let processesPerPage = $state(settings.processesPerPage);
  let lastPage = $derived(Math.ceil(processes.length / processesPerPage));
  let dropDownOpen = $state(false);

  $effect(() => {
    if (pageNumber > lastPage) {
      if (lastPage === 0) {
        setPageNumber(1);
      } else {
        setPageNumber(lastPage);
      }
    } else if (pageNumber < 1) {
      setPageNumber(1);
    }
  });

  function handleAmountPerPageChange(newAmount: number) {
    processesPerPage = newAmount;
    settings.processesPerPage = newAmount;
    dropDownOpen = false;
  }

  function nextPage() {
    if (pageNumber >= lastPage) return;
    setPageNumber(pageNumber + 1);
  }

  function previousPage() {
    if (pageNumber <= 1) return;
    setPageNumber(pageNumber - 1);
  }

  function goToFirstPage() {
    setPageNumber(1);
  }

  function goToLastPage() {
    setPageNumber(lastPage);
  }
</script>

<div class="flex items-center gap-2">
  <button id="page-dropdown" class="group rounded-md bg-secondary p-1.5 border border-text-secondary border-opacity-50 hover:border-opacity-100 text-text-secondary hover:text-text-primary text-xs pl-2 pr-2 flex items-center justify-center gap-1.5">
    <p class="whitespace-nowrap">{processesPerPage} per page</p>
    <Fa icon={faChevronDown} class="text-text-secondary group-hover:text-text-primary" />
  </button>
  <Dropdown bind:open={dropDownOpen} triggeredBy="#page-dropdown" class="min-w-24 border-[1px] border-text-secondary bg-secondary rounded-md text-text-secondary w-full">
    <DropdownItem onclick={() => handleAmountPerPageChange(15)} class="p-2 text-xs hover:text-text-primary">15 per page</DropdownItem>
    <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
    <DropdownItem onclick={() => handleAmountPerPageChange(30)} class="p-2 text-xs hover:text-text-primary">30 per page</DropdownItem>
    <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
    <DropdownItem onclick={() => handleAmountPerPageChange(45)} class="p-2 text-xs hover:text-text-primary">45 per page</DropdownItem>
  </Dropdown>
  <button onclick={goToFirstPage} disabled={pageNumber <= 1} class="{pageNumber <= 1 ? 'opacity-50 hover:cursor-not-allowed' : 'hover:border-opacity-100 hover:text-text-primary'} group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 text-text-secondary h-7 w-7 flex items-center justify-center">
    ««
  </button>
  <button onclick={previousPage} disabled={pageNumber <= 1} class="{pageNumber <= 1 ? 'opacity-50 hover:cursor-not-allowed' : 'hover:border-opacity-100 hover:text-text-primary'} group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 text-text-secondary h-7 w-7 flex items-center justify-center">
    «
  </button>
  <div class="whitespace-nowrap flex flex-col items-center text-xs text-text-secondary">
    <p>Page {pageNumber} of {lastPage || 1}</p>
    <p class="opacity-70">({processes.length} processes)</p>
  </div>
  <button onclick={nextPage} disabled={pageNumber >= lastPage} class="{pageNumber >= lastPage ? 'opacity-50 hover:cursor-not-allowed' : 'hover:border-opacity-100 hover:text-text-primary'} group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 text-text-secondary h-7 w-7 flex items-center justify-center">
    »
  </button>
  <button onclick={goToLastPage} disabled={pageNumber >= lastPage} class="{pageNumber >= lastPage ? 'opacity-50 hover:cursor-not-allowed' : 'hover:border-opacity-100 hover:text-text-primary'} group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 text-text-secondary h-7 w-7 flex items-center justify-center">
    »»
  </button>
</div>
