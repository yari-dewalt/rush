import type { Settings } from "$lib/types";
import { browser } from "$app/environment";

export function createSettings() {
  const defaultSettings: Settings = {
    interval: 1000,
    processesPerPage: 15,
    theme: 'Dark',
    showWarningPopup: true,
  };

  let storedSettings: Settings;
  try {
    if (browser) {
      const stored = window.localStorage.getItem("settings");
      storedSettings = stored ? JSON.parse(stored) : defaultSettings;
    } else {
      storedSettings = defaultSettings;
    }
  } catch (e) {
    console.warn('Failed to parse stored settings:', e);
    storedSettings = defaultSettings;
  }

  const settings: Settings = $state(storedSettings);

  // Save settings on changes
  $effect(() => {
    localStorage.setItem("settings", JSON.stringify(settings));
  });

  return {
    get interval() { return settings.interval },
    set interval(value) { settings.interval = value },
    get processesPerPage() { return settings.processesPerPage },
    set processesPerPage(value) { settings.processesPerPage = value },
    get theme() { return settings.theme },
    set theme(value) { settings.theme = value },
    get showWarningPopup() { return settings.showWarningPopup },
    set showWarningPopup(value) { settings.showWarningPopup = value },
  };
}
