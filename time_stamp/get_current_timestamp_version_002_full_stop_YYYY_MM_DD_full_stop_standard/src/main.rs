use chrono::Local;

fn main() {
    // Get current local date/time
    let now = Local::now();

    // Format as YYYY_MM_DD
    let formatted_date = now.format("%Y_%m_%d").to_string();

    println!("Current Date: {}", formatted_date);
}