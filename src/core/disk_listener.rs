use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{DiskExt, SystemExt};
use tauri::{Manager, Runtime, Window};
use super::*;

static RUN_LISTENER: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
struct DiskUpdatePayload {
    id: char,
    space_now: u64,
}

#[tauri::command]
pub fn stop_disk_listener() {
    RUN_LISTENER.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn spawn_disk_listener<R: Runtime>(window: Window<R>) {
    // check if listener is running and if not mark as running
    if RUN_LISTENER.load(Ordering::SeqCst) == true {
        return;
    } else {
        RUN_LISTENER.store(true, Ordering::SeqCst);
    }

    std::thread::spawn(move || {
        let mut sys = create_disk_system();
        let mut disk_size_map = HashMap::<char, u64>::new();

        // run listener while the flag is true
        while RUN_LISTENER.load(Ordering::SeqCst) {
            sys.refresh_disks();

            for disk in sys.disks() {
                if let Some(id) = disk
                    .mount_point()
                    .get_disk_letter()
                {
                    let space_now = disk_size_map
                        .entry(id)
                        .or_insert_with(|| 0);

                    let space_new = disk.available_space();

                    if *space_now != space_new {
                        *space_now = space_new;

                        window.emit("disk_update", DiskUpdatePayload {
                            id,
                            space_now: space_new,
                        }).unwrap();
                    }
                }
            }

            sleep(Duration::from_secs(1));
        }
    });
}
