use rust_functions::number_formatting::format_number;

fn main() {
    // Default group size (3) when passing ""
    println!("{}", format_number("1234567", "")); 
    // → "001_234_567"

    // Explicit group size as string
    println!("{}", format_number("1234567", "4")); 
    // → "0123_4567"

    // Explicit group size as usize
    println!("{}", format_number("1234567", 4)); 
    // → "0123_4567"

    // Handles decimals
    println!("{}", format_number("1234.5678", 3)); 
    // → "001_234_decimal_point_567_800"

    // Handles negatives
    println!("{}", format_number("-1234", "")); 
    // → "negative_001_234"
}