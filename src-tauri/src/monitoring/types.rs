use serde::Serialize;

#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: Vec<f32>,
    pub total_memory: u64,
    pub free_memory: u64,
    pub used_memory: u64,
    pub uptime: u64,
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_free_bytes: u64,
    pub disk_read: u64,
    pub disk_write: u64,
    pub disks: Vec<DiskInfo>,
    //pub network_rx_bytes: u64,
    //pub network_tx_bytes: u64,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub root: String,
    pub memory_usage: u64,
    pub status: String,
    pub start_time: u64,
    pub run_time: u64,
    pub cpu_usage: f32,
    pub disk_usage: (u64, u64),
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub kind: String,
    pub total_space: u64,
    pub available_space: u64,
    pub usage: (u64, u64), // (read_bytes, write_bytes)
}
