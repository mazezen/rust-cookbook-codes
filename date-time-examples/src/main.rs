use std::println;

use time::{OffsetDateTime, UtcOffset};

fn main() -> Result<(), time::error::ComponentRange> {
    let now = OffsetDateTime::now_utc();
    println!("UTC time: {now}");

    let tokyo = now.to_offset(UtcOffset::from_hms(9, 0, 0)?);
    println!("UTC+09:00:  {tokyo}");

    println!(
        "Year {}, month {}, day {}, weekday {}",
        now.year(),
        now.month(),
        now.day(),
        now.weekday()
    );
    println!(
        "Time {:02}:{:02}:{:02}",
        now.hour(),
        now.minute(),
        now.second()
    );

    Ok(())
}
