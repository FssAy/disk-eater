#[macro_use]
extern crate serde;

mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            core::get_disk_info,
            core::spawn_disk_listener,
            core::stop_disk_listener,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
