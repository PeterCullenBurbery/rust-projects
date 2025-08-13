use chrono::Local;

fn main() {
    // Get current date and time with time zone offset
    let timestamp = Local::now();

    println!("Current Timestamp: {}", timestamp);
}