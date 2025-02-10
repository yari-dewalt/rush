<script lang="ts">
  import type { Process, Settings, SortDirection } from '$lib/types.ts';
  import ProcessesToolbar from './ProcessesToolbar.svelte';
  import ProcessesList from './ProcessesList.svelte';
  import { getContext } from 'svelte';
	import { pinnedProcesses } from '$lib/state/pinnedProcesses.svelte';

  let { processes: initialProcesses }: { processes: Process[] } = $props();
  const settings: Settings = getContext('settings');
  let searchValue = $state('');
  let attribute = $state<keyof Process>('pid');
  let direction = $state<SortDirection>('upDown');
  let pageNumber = $state(1);
  let status = $state('All Statuses');

  const filteredProcesses = $derived.by(() => {
    if (!initialProcesses) return [];
    let result = [...initialProcesses];

    // Apply regex search
    if (searchValue) {
      try {
        const regex = new RegExp(searchValue, 'gi');
        result = result.filter(process => regex.test(process.name));
      } catch (e) {
        // If regex is invalid, return all processes
        console.warn('Invalid regex pattern:', e);
      }
    }

    // Apply sort
    if (attribute && direction !== 'upDown') {
      result = sortProcesses(result, attribute, direction, status);
    } else {
      result = result.filter(process => {
        if (status === 'All Statuses') return true;
        return process.status === status;
      });
    }

    return result;
  });

  const displayedProcesses = $derived.by(() => {
    // Get pinned processes
    const pinnedProcs = initialProcesses.filter(p => pinnedProcesses.has(p.pid));
    
    // Get paginated non-pinned processes
    const nonPinnedProcs = filteredProcesses
      .filter(p => !pinnedProcesses.has(p.pid))
      .slice(
        (pageNumber - 1) * settings.processesPerPage, 
        pageNumber * settings.processesPerPage
      );
    
    return [...pinnedProcs, ...nonPinnedProcs];
  });

  function setSearchValue(newValue: string) {
    searchValue = newValue;
  }

  function setSortOrder(newAttribute: keyof Process, newDirection: SortDirection) {
    attribute = newAttribute;
    direction = newDirection;
  }

  function setStatus(newStatus: string) {
    status = newStatus;
  };

  function sortProcesses (processes: Process[], attribute: keyof Process, direction: SortDirection, status: string) {
    if (direction === 'upDown') return processes;
    
    let sortedProcesses = [...processes].sort((a: Process, b: Process) => {
      const aVal = a[attribute];
      const bVal = b[attribute];
      
      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return direction === 'asc' ? aVal - bVal : bVal - aVal;
      }
      
      const aStr = String(aVal).toLowerCase();
      const bStr = String(bVal).toLowerCase();
      return direction === 'asc' 
        ? aStr.localeCompare(bStr)
        : bStr.localeCompare(aStr);
    });

    sortedProcesses = sortedProcesses.filter(process => {
      if (status === 'All Statuses') return true;
      return process.status === status;
    });

    return sortedProcesses;
  }

  function setPageNumber(newPageNumber: number) {
    pageNumber = newPageNumber;
  }
</script>

<div class="flex flex-col">
  <ProcessesToolbar processes={filteredProcesses} {setSortOrder} {setSearchValue} pageNumber={pageNumber} {setPageNumber} {setStatus} />
  <ProcessesList processes={displayedProcesses} />
</div>
