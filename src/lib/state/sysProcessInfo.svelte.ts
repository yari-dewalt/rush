import type { Process, SystemInfo } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export function createSysProcessInfo() {
  const sysProcessInfo = $state({
    processes: [] as Process[],
    systemInfo: {} as SystemInfo,
  });

  const getInfo = async () => {
    const info: [Process[], SystemInfo] = await invoke('get_info');
    sysProcessInfo.processes = info[0];
    sysProcessInfo.systemInfo = info[1];
  }

  return {
    getInfo,
    get processes() { return sysProcessInfo.processes },
    get systemInfo() { return sysProcessInfo.systemInfo },
  };
}
