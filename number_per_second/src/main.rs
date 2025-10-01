use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time,
};
use rust_functions::number_formatting::format_number;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle Ctrl+C
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut counter: u64 = 1;
    while running.load(Ordering::SeqCst) {
        let num_str = counter.to_string();
        let formatted = format_number(&num_str, "");
        println!("{}", formatted);

        counter += 1;
        // Sleep 1 millisecond instead of 1 second
        thread::sleep(time::Duration::from_millis(1));
    }

    println!("Exiting.");
}