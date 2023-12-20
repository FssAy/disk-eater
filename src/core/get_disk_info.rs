use super::*;
use sysinfo::{RefreshKind, System, SystemExt};

#[tauri::command]
pub fn get_disk_info() -> Vec<Disk> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_disks()
            .with_disks_list()
    );

    sys.refresh_disks();

    let disks = sys
        .disks()
        .iter()
        .map(Disk::from)
        .collect::<Vec<Disk>>();

    disks
}
