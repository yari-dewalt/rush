import { invoke } from "@tauri-apps/api/core";

export function createSysProcessInfo() {
  let sysProcessInfo = $state({
    processes: [],
    systemInfo: {}
  });

  const getInfo = async () => {
      const info = await invoke('get_info');
      sysProcessInfo.processes = info[0];
      sysProcessInfo.systemInfo = info[1];
  }

  return {
    getInfo,
    get processes() { return sysProcessInfo.processes},
    get systemInfo() { return sysProcessInfo.systemInfo },
  };
}
