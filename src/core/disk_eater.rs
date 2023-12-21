use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use tauri::{Manager, Window};
use super::*;

static RUN_EATER: AtomicBool = AtomicBool::new(false);

const BUFFER_SIZE: usize = 100_000_000;
const FILE_REFRESH_SIZE: usize = 500_000_000;

#[tauri::command]
pub fn stop_disk_eater() {
    RUN_EATER.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn spawn_disk_eater(window: Window, ids: Vec<char>) -> Result<(), String> {
    if RUN_EATER.load(Ordering::SeqCst) {
        return Err("eater is already running".to_string());
    }

    let mut files = Vec::with_capacity(ids.len());
    for id in ids {
        let mut path = PathBuf::new().join(
            format!("{}:/DiskEater", id)
        );

        if !path.exists() {
            create_dir_all(&path).map_err(|e| e.to_string())?;
        }

        path = path.join(format!(
            "{}.bin",
            random_string::generate(18, random_string::charsets::ALPHANUMERIC)
        ));

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        files.push((id, file));
    }

    RUN_EATER.store(true, Ordering::SeqCst);

    let window = Arc::new(window);

    for (id, file) in files {
        let runner = Arc::new(&RUN_EATER);

        let window = Arc::clone(&window);
        thread::spawn(move || {
            let mut file = file;
            let mut file_size_state = 0;
            let mut file_size_refresh = FILE_REFRESH_SIZE;

            let mut buffer = vec![0u8; BUFFER_SIZE];
            let mut buffer_cap_size = BUFFER_SIZE;

            let mut rng = SmallRng::from_entropy();

            while runner.load(Ordering::SeqCst) && buffer_cap_size > 0 {
                let mut buffer = &mut buffer[..buffer_cap_size];

                match file_filler(&mut file, &mut buffer, &mut rng) {
                    Some(size) => file_size_state += size,
                    None => {
                        /*
                            ToDo: run this code only when there is no more space on disk
                            for other error -> break
                        */
                        file_size_refresh = 0;
                        buffer_cap_size /= 2;
                        continue;
                    },
                }

                if file_size_state >= file_size_refresh {
                    if file.sync_all().is_err() {
                        break;
                    }

                    file_size_state = 0;
                }
            }

            if Arc::strong_count(&runner) <= 1 {
                runner.store(false, Ordering::SeqCst);
                send_finish_signal(window.as_ref());
            }
        });
    }

    Ok(())
}

fn file_filler(file: &mut File, buffer: &mut [u8], rng: &mut SmallRng) -> Option<usize> {
    rng.fill(buffer);
    let result = file.write(&buffer);

    // clear
    let _ = buffer.fill(0);

    result.ok()
}

fn send_finish_signal(window: &Window) {
    window.emit(
        "eating_finished",
        None::<()>,
    ).ok();
}
