use super::*;
use sysinfo::{SystemExt};

#[tauri::command]
pub fn get_disk_info() -> Vec<Disk> {
    let mut sys = create_disk_system();

    sys.refresh_disks();

    let disks = sys
        .disks()
        .iter()
        .map(Disk::from)
        .collect::<Vec<Disk>>();

    disks
}
