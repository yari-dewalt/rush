use super::SystemStats;

pub struct SystemMonitor {
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn collect_stats(&mut self, sys: &sysinfo::System) -> SystemStats {
        SystemStats {
            cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            total_memory: sys.total_memory(),
            free_memory: sys.free_memory(),
            used_memory: sys.used_memory(),
            uptime: sysinfo::System::uptime(),
        }
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
