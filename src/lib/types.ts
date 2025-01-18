export interface SystemInfo {
    cpu_usage: number[],
    total_memory: number,
    free_memory: number,
    used_memory: number,
    uptime: number,
    disk_total_bytes: number,
    disk_used_bytes: number,
    disk_free_bytes: number,
    disk_read: number,
    disk_write: number,
    disks: Disk[],
    network_received: number,
    network_transmitted: number,
    network_total_received: number,
    network_total_transmitted: number,
};

export interface MemoryData {
    totalMemory: number,
    freeMemory: number,
    usedMemory: number,
};

export interface StorageData {
    total: number,
    free: number,
    used: number,
    read: number,
    write: number,
};

export interface NetworkData {
  received: number,
  transmitted: number,
  totalReceived: number,
  totalTransmitted: number,
};

export interface Disk {
    name: string,
    kind: string,
    totalSpace: number,
    availableSpace: number,
    readUsage: number,
    writeUsage: number,
};
