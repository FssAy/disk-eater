#[macro_use]
extern crate serde;

mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            core::get_disk_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
