use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;
fn main() {
    let system_time = Local::now();
    let utc_time = Utc::now();

    // println!("{}", system_time.date_naive());
    // println!("{}", utc_time.date_naive());

    // println!("{}", system_time.time());
    // println!("{}", utc_time.time());

    // println!("{}", system_time.year());
    // println!("{}", utc_time.year());

    let local_time = Local::now();
    let la_time = local_time.with_timezone(&Los_Angeles);
    println!("{}", la_time);
}
