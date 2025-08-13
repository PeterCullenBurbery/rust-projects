use std::env;

fn main() {
    // Collect all CLI args after the binary name.
    let args: Vec<String> = env::args().skip(1).collect();

    // Use provided args joined by spaces, or the default.
    let input_value = if args.is_empty() {
        String::from("the trash")
    } else {
        args.join(" ")
    };

    fucking_focus(&input_value);
}

fn fucking_focus(value: &str) {
    // Keeps capitalization exactly as provided.
    println!("It's time to stop fucking around with {}.", value);
    println!("It's time to take {} seriously.", value);
    println!("It's time to stop fucking around with {}, and take {} seriously. It's time to put a concerted effort into {}.", value, value, value);
    println!("It's time to stop fucking around with {} and put some time into {}.", value, value);
}