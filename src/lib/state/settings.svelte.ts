import type { Settings } from "$lib/types";

export function createSettings() {
  const settings: Settings = $state({
    interval: 1000,
    processesPerPage: 15,
    theme: 'Dark',
  });

  return {
    get interval() { return settings.interval },
    set interval(value) { settings.interval = value },
    get processesPerPage() { return settings.processesPerPage },
    set processesPerPage(value) { settings.processesPerPage = value },
    get theme() { return settings.theme },
    set theme(value) { settings.theme = value },
  };
}
