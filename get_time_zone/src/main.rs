fn main() {
    match iana_time_zone::get_timezone() {
        Ok(tz) => println!("Local time zone: {}", tz),
        Err(e) => eprintln!("Could not determine local time zone: {}", e),
    }
}