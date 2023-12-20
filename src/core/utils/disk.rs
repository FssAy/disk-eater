use sysinfo::DiskExt;
use crate::core::utils::{GetDiskLetter, OsStringConv};

#[derive(Serialize)]
pub struct Disk {
    name: String,
    mount: Option<char>,
    removable: bool,
    space_max: u64,
    space_now: u64,
}

impl From<&sysinfo::Disk> for Disk {
    fn from(disk: &sysinfo::Disk) -> Self {
        Self {
            name: disk.name().to_string(),
            mount: disk.mount_point().get_disk_letter(),
            removable: disk.is_removable(),
            space_max: disk.total_space(),
            space_now: disk.available_space(),
        }
    }
}
