use chrono::Local;

fn main() {
    // Get current local date/time
    let now = Local::now();

    // Format as YYYY_MM_DD_HH_mm_ss
    let formatted_timestamp = now.format("%Y_%m_%d_%H_%M_%S").to_string();

    println!("Current Timestamp: {}", formatted_timestamp);
}