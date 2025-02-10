use crate::monitoring::{ProcessMonitor, SystemMonitor};
use std::sync::Mutex;
use sysinfo::System;

pub struct AppState {
    pub sys: Mutex<System>,
    pub process_monitor: Mutex<ProcessMonitor>,
    pub system_monitor: Mutex<SystemMonitor>,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        Self {
            sys: Mutex::new(sys),
            process_monitor: Mutex::new(ProcessMonitor::new()),
            system_monitor: Mutex::new(SystemMonitor::new()),
        }
    }
}
