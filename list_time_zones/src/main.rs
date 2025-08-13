use chrono_tz::Tz;

fn main() {
    for tz in chrono_tz::TZ_VARIANTS {
        println!("{}", tz.name());
    }
}