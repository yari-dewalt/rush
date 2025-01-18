use super::{SystemStats, DiskInfo};
use sysinfo::{Disk, Disks};
use std::path::Path;

pub struct SystemMonitor {
    disks: Disks
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn collect_stats(&mut self, sys: &sysinfo::System) -> SystemStats {
        let disks = Self::filter_disks(&self.disks);
        let mut disk_vec = Vec::with_capacity(disks.len());
        let mut disk_total_bytes = 0;
        let mut disk_free_bytes = 0;
        let mut disk_read = 0;
        let mut disk_write = 0;

        for disk in disks {
            let disk_usage = disk.usage();
            disk_vec.push(DiskInfo {
                name: disk.name().to_str().unwrap().to_string(),
                kind: disk.kind().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                usage: (disk_usage.read_bytes, disk_usage.written_bytes),
            });

            disk_total_bytes += disk.total_space();
            disk_free_bytes += disk.available_space();
            disk_read += disk_usage.read_bytes;
            disk_write += disk_usage.written_bytes;
        }

        self.disks.refresh(true);

        SystemStats {
            cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            total_memory: sys.total_memory(),
            free_memory: sys.free_memory(),
            used_memory: sys.used_memory(),
            uptime: sysinfo::System::uptime(),
            disk_total_bytes,
            disk_used_bytes: disk_total_bytes - disk_free_bytes,
            disk_free_bytes,
            disk_read,
            disk_write,
            disks: disk_vec,
        }
    }

    /// Filters disks based on platform-specific criteria
    #[cfg(not(target_os = "windows"))]
    fn filter_disks(disks: &[Disk]) -> Vec<&Disk> {
        disks
            .iter()
            .filter(|disk| disk.mount_point() == Path::new("/"))
            .collect()
    }

    /// Windows-specific disk filtering
    #[cfg(target_os = "windows")]
    fn filter_disks(disks: &[Disk]) -> Vec<&Disk> {
        disks.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_stats_collection() {
        let mut sys = System::new();
        let mut monitor = SystemMonitor::new();
        sys.refresh_all();

        let stats = monitor.collect_stats(&sys);
        assert!(!stats.cpu_usage.is_empty());
        assert!(stats.total_memory > 0);
    }
}
