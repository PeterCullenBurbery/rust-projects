use chrono::{Local, Timelike, Datelike};
use hostname::get;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, fs};

fn format_label(time: chrono::DateTime<Local>, epoch_secs: u64) -> String {
    let ns = time.nanosecond();
    let ms = ns / 1_000_000;
    let us = (ns / 1_000) % 1_000;
    let ns_rem = ns % 1_000;

    format!(
        "year_is_{:04}_month_is_{:03}_day_is_{:03}_hour_is_{:03}_minute_is_{:03}_{:03}_{:03}_{:03}_{:03}_{}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second(),
        ms,
        us,
        ns_rem,
        epoch_secs
    )
}

fn main() {
    let duration_secs: u64 = 1200; // adjustable segment length

    // === Computer Name ===
    let computer_name = get().unwrap().to_string_lossy().into_owned();

    // === Shared state for Ctrl+C ===
    let running = Arc::new(AtomicBool::new(true));
    let child_process: Arc<Mutex<Option<std::process::Child>>> = Arc::new(Mutex::new(None));
    let stopped_early = Arc::new(AtomicBool::new(false));
    let actual_end: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    {
        let running = running.clone();
        let child_process = child_process.clone();
        let stopped_early = stopped_early.clone();
        let actual_end = actual_end.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
            stopped_early.store(true, Ordering::SeqCst);

            // compute actual end label
            let now = Local::now();
            let now_secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let end_label = format_label(now, now_secs);

            let mut lock = actual_end.lock().unwrap();
            *lock = Some(end_label);

            if let Ok(mut child_opt) = child_process.lock() {
                if let Some(child) = child_opt.as_mut() {
                    let _ = child.kill();
                }
            }
        })
        .expect("Error setting Ctrl-C handler");
    }

    println!(
        "Recording in {} second segments. Press Ctrl+C to stop early.",
        duration_secs
    );

    // === Main loop: run segment after segment until Ctrl+C ===
    let mut start_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    while running.load(Ordering::SeqCst) {
        let start = Local::now();
        let start_label = format_label(start, start_secs);

        // compute next boundary
        let planned_end_secs = start_secs + duration_secs;
        let planned_end = start + chrono::Duration::seconds(duration_secs as i64);
        let planned_end_label = format_label(planned_end, planned_end_secs);

        let planned_filename = format!(
            "C:/data/videos/{}_video_starting_at_{}_and_ending_at_{}.mp4",
            computer_name, start_label, planned_end_label
        );

        println!("Recording desktop to: {}", planned_filename);

        // === Spawn ffmpeg for this segment ===
        let child = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-f", "gdigrab",
                "-framerate", "30",
                "-t", &duration_secs.to_string(),
                "-i", "desktop",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                &planned_filename,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start ffmpeg");

        {
            let mut lock = child_process.lock().unwrap();
            *lock = Some(child);
        }

        // Wait until process exits or Ctrl+C pressed
        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_millis(200));
            let finished = {
                let mut lock = child_process.lock().unwrap();
                if let Some(child) = lock.as_mut() {
                    child.try_wait().unwrap().is_some()
                } else {
                    true
                }
            };
            if finished {
                break;
            }
        }

        if let Some(mut child) = child_process.lock().unwrap().take() {
            let _ = child.wait();
        }

        // If stopped early, rename file with actual end timestamp
        if stopped_early.load(Ordering::SeqCst) {
            if let Some(end_label) = actual_end.lock().unwrap().clone() {
                let new_filename = format!(
                    "C:/data/videos/{}_video_starting_at_{}_and_ending_at_{}.mp4",
                    computer_name, start_label, end_label
                );
                let _ = fs::rename(&planned_filename, &new_filename);
                println!("Recording stopped early. Saved to {}", new_filename);
            }
            break; // exit loop on Ctrl+C
        }

        println!("Recording completed. Saved to {}", planned_filename);

        // advance to next segment
        start_secs = planned_end_secs;
    }
}