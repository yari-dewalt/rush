use super::ProcessInfo;
use std::path::Path;
use sysinfo::{Pid, ProcessStatus};

pub struct ProcessMonitor {}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect_processes(&mut self, sys: &sysinfo::System) -> Result<Vec<ProcessInfo>, String> {
        let process_data = self.build_process_data(sys);
        Ok(process_data)
    }

    fn build_process_data(&self, sys: &sysinfo::System) -> Vec<ProcessInfo> {
        sys.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                ppid: process.parent().unwrap_or(Pid::from(0)).as_u32(),
                name: process.name().to_string_lossy().into_owned(),
                user_id: process
                    .user_id()
                    .map(|uid| uid.to_string())
                    .unwrap_or("".to_string()),
                exe: process
                    .exe()
                    .unwrap_or(Path::new(""))
                    .to_string_lossy()
                    .into_owned(),
                cmd: process
                    .cmd()
                    .iter()
                    .map(|s| s.to_string_lossy().into_owned())
                    .collect::<Vec<String>>()
                    .join(" "),
                cpu_usage: process.cpu_usage(),
                disk_usage: (
                    process.disk_usage().read_bytes,
                    process.disk_usage().written_bytes,
                ),
                start_time: process.start_time(),
                run_time: process.run_time(),
                memory_usage: process.memory(),
                virtual_memory: process.virtual_memory(),
                status: Self::format_status(process.status()),
            })
            .collect()
    }

    fn format_status(status: ProcessStatus) -> String {
        match status {
            ProcessStatus::Run => "Running",
            ProcessStatus::Sleep => "Sleeping",
            ProcessStatus::Idle => "Idle",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn kill_process(sys: &sysinfo::System, pid: u32) -> bool {
        sys.process(sysinfo::Pid::from(pid as usize))
            .map(|process| process.kill())
            .unwrap_or(false)
    }
}
