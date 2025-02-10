use crate::monitoring::{ProcessInfo, ProcessMonitor, SystemStats};
use crate::state::AppState;
use sysinfo::ProcessesToUpdate;
use tauri::State;

#[tauri::command]
pub async fn get_info(
    state: State<'_, AppState>,
) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    sys.refresh_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut process_monitor = state.process_monitor.lock().map_err(|e| e.to_string())?;
    let mut system_monitor = state.system_monitor.lock().map_err(|e| e.to_string())?;

    let processes = process_monitor.collect_processes(&sys)?;
    let system_stats = system_monitor.collect_stats(&sys);

    Ok((processes, system_stats))
}

#[tauri::command]
pub async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|e| e.to_string())?;
    Ok(ProcessMonitor::kill_process(&sys, pid))
}
