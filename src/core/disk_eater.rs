use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use super::*;

static RUN_EATER: AtomicBool = AtomicBool::new(false);

const BUFFER_SIZE: usize = 1_000_000;
const FILE_REFRESH_SIZE: usize = 50_000_000;

type Buffer = [u8; BUFFER_SIZE];

#[tauri::command]
pub fn stop_disk_eater() {
    RUN_EATER.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn spawn_disk_eater(ids: Vec<char>) -> Result<(), String> {
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

        files.push(file);
    }

    RUN_EATER.store(true, Ordering::SeqCst);

    for file in files {
        let runner = Arc::new(&RUN_EATER);

        thread::spawn(move || {
            let mut file = file;
            let mut rng = SmallRng::from_entropy();
            let mut buffer = [0u8; BUFFER_SIZE];
            let mut file_size_state = 0;

            while runner.load(Ordering::SeqCst) {
                match file_filler(&mut file, &mut buffer, &mut rng) {
                    Some(size) => file_size_state += size,
                    None => break,
                }

                if file_size_state >= FILE_REFRESH_SIZE {
                    if file.sync_all().is_err() {
                        break;
                    }

                    file_size_state = 0;
                }

                sleep(Duration::from_millis(100));
            }

            if Arc::strong_count(&runner) <= 1 {
                runner.store(false, Ordering::SeqCst);
            }
        });
    }

    Ok(())
}

fn file_filler(file: &mut File, buffer: &mut Buffer, rng: &mut SmallRng) -> Option<usize> {
    let buffer = buffer as &mut [u8];
    rng.fill(buffer);
    file.write(&buffer).ok()
}
