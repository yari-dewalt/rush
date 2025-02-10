<script lang="ts">
  import type { Process, SortDirection } from '$lib/types.ts';
  import { faGear, faChevronDown } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import { Dropdown, DropdownItem, DropdownDivider } from 'flowbite-svelte';
  import ToolbarHeader from './ToolbarHeader.svelte';
  import PageHandler from './PageHandler.svelte';
  import { popupStore } from "$lib/state/popupStore.svelte";

  type ArrowState = 'upDown' | 'asc' | 'desc';
  type Props = {
    processes: Process[],
    setSortOrder: (newAttribute: keyof Process, newDirection: SortDirection) => void,
    setSearchValue: (value: string) => void,
    pageNumber: number,
    setPageNumber: (pageNumber: number) => void,
    setStatus: (status: string) => void,
  }
  
  let { processes, setSortOrder, setSearchValue, pageNumber, setPageNumber, setStatus }: Props = $props();

  let searchValue = $state('');

  let states = $state({
    name: 'upDown',
    pid: 'upDown',
    status: 'upDown',
    cpu_usage: 'upDown',
    memory_usage: 'upDown',
    virtual_memory: 'upDown',
    disk_usage: 'upDown',
    run_time: 'upDown',
  }) as Record<string, ArrowState>;

  let status = $state('All Statuses');
  let statusDropDownOpen = $state(false);

  function clearOtherStates(currentHeader: string) {
    for (const header in states) {
      if (header !== currentHeader) {
        states[header] = 'upDown';
      }
    }
  }

  function handleHeaderClick(header: string) {
    clearOtherStates(header);
    const newDirection = states[header] === 'upDown' ? 'desc' : 
                    states[header] === 'desc' ? 'asc' : 'upDown';
    states[header] = newDirection;
    setSortOrder(header as keyof Process, newDirection);
  }

  function handleChange() {
    setSearchValue(searchValue);
    setPageNumber(1);
  }

  function updateStatus(newStatus: string) {
    status = newStatus;
    setStatus(newStatus);
    statusDropDownOpen = false;
  }

  function openSettingsPopup() {
    popupStore.visible = true;
    popupStore.popup = 'settings';
  }
</script>

<div class="flex flex-col gap-1.5 bg-primary pt-1.5 pb-1.5">
  <div class="flex items-center gap-2 pl-4 pr-4">
    <div class="flex items-center gap-2">
      <input bind:value={searchValue} oninput={handleChange} class="rounded-lg p-1.5 border border-1 border-text-secondary border-opacity-50 focus:outline-none focus:border-opacity-100 pl-3 pr-3 text-xs bg-secondary text-text-primary placeholder:text-text-secondary" placeholder="Search processes" />
      <button id="statuses-dropdown" class="group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 hover:border-opacity-100 text-text-secondary hover:text-text-primary text-xs pl-2 pr-2 flex items-center justify-center gap-1.5">
        <p class="whitespace-nowrap">{status}</p>
        <Fa icon={faChevronDown} class="text-text-secondary group-hover:text-text-primary" />
      </button>
      <Dropdown bind:open={statusDropDownOpen} triggeredBy="#statuses-dropdown" class="min-w-24 border border-text-secondary bg-secondary rounded-md text-text-secondary w-full">
        <DropdownItem onclick={() => updateStatus('All Statuses')} class="p-2 text-xs hover:text-text-primary">All Statuses</DropdownItem>
        <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
        <DropdownItem onclick={() => updateStatus('Running')} class="p-2 text-xs hover:text-text-primary">Running</DropdownItem>
        <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
        <DropdownItem onclick={() => updateStatus('Idle')} class="p-2 text-xs hover:text-text-primary">Idle</DropdownItem>
        <DropdownDivider class="h-[1px] w-full bg-text-secondary opacity-80" />
        <DropdownItem onclick={() => updateStatus('Sleeping')} class="p-2 text-xs hover:text-text-primary">Sleeping</DropdownItem>
      </Dropdown>
    </div>
    <div class="flex items-center justify-center ml-auto mr-auto">
      <PageHandler {processes} {pageNumber} {setPageNumber} />
    </div>
    <div class="flex items-center justify-end ml-auto">
      <button onclick={openSettingsPopup} class="group rounded-md bg-secondary p-1.5 border border-1 border-text-secondary border-opacity-50 hover:border-opacity-100">
        <Fa icon={faGear} size="sm" class="text-text-secondary group-hover:text-text-primary" />
      </button>
    </div>
  </div>
  <div class="min-h-[1px] w-full bg-secondary"></div>
  <div class="flex items-center pl-2 pr-2 text-sm text-text-secondary">
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="Process Name" arrowState={states.name} clickHandler={() => handleHeaderClick('name')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="PID" arrowState={states.pid} clickHandler={() => handleHeaderClick('pid')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="Status" arrowState={states.status} clickHandler={() => handleHeaderClick('status')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="CPU %" arrowState={states.cpu_usage} clickHandler={() => handleHeaderClick('cpu_usage')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="RAM" arrowState={states.memory_usage} clickHandler={() => handleHeaderClick('memory_usage')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="Virtual Memory" arrowState={states.virtual_memory} clickHandler={() => handleHeaderClick('virtual_memory')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="Disk R/W" arrowState={states.disk_usage} clickHandler={() => handleHeaderClick('disk_usage')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%]">
      <ToolbarHeader text="Runtime" arrowState={states.run_time} clickHandler={() => handleHeaderClick('run_time')}/>
    </div>
    <div class="min-w-[11.1%] max-w-[11.1%] flex items-center justify-center">
      <p class="hover:text-text-primary">Actions</p>
    </div>
  </div>
</div>
