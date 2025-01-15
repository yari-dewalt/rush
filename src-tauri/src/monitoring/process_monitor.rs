use super::ProcessInfo;
use std::path::Path;
use sysinfo::Pid;

pub struct ProcessMonitor {
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn collect_processes(&mut self, sys: &sysinfo::System) -> Result<Vec<ProcessInfo>, String> {
        let process_data = self.build_process_data(sys);
        Ok(process_data)
    }

    fn build_process_data(&self, sys: &sysinfo::System) -> Vec<ProcessInfo> {
        sys.processes()
            .iter()
            .map(|(pid, process)| {
                ProcessInfo {
                    pid: pid.as_u32(),
                    ppid: process.parent().unwrap_or(Pid::from(0)).as_u32(),
                    name: process.name().to_string_lossy().into_owned(),
                    root: process.root().unwrap_or(Path::new("")).to_string_lossy().into_owned(),
                    cpu_usage: process.cpu_usage(),
                    disk_usage: (process.disk_usage().read_bytes, process.disk_usage().written_bytes),
                    start_time: process.start_time(),
                    run_time: process.run_time(),
                    memory_usage: process.memory(),
                    status: process.status().to_string(),
                }
            })
            .collect()

    }
}
