use rust_functions::number_formatting::format_number;

fn main() {
    let n = "1234567.0";
    let formatted = format_number(n, "3");
    println!("Formatted: {}", formatted);
}