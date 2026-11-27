// use std::println;

// use jiff::{Timestamp, ZonedDateTime};

// fn main() -> Result<(), jiff::Error> {
//     let now = ZonedDateTime::now();
//     println!("Local time: {now}");

//     let tokyo = now.in_tz("Asiz/Tokyo")?;
//     println!("In Tokyo: {tokyo}");
//     println!("UTC offset: {}", tokyo.offset());

//     let timestamp: Timestamp = now.timestamp();
//     println!("Timestamp: {timestamp}");

//     println!(
//         "Year {}, month {}, day {}, weekday {:?}",
//         now.year(),
//         now.month(),
//         now.day(),
//         now.weekday()
//     );
//     println!(
//         "Time {:02}:{:02}:{:02}",
//         now.hour(),
//         now.minute(),
//         now.second()
//     );

//     Ok(())
// }

use std::println;

use jiff::ZonedDateTime;
use jiff::civil::Date;
use jiff::fmt::strtime;

fn main() -> Result<(), jiff::Error> {
    let zoned: ZonedDateTime = "2024-07-11T01:14:00-04:00[America/New_York]".parse()?;
    println!("Parsed: {zoned}");
    println!(
        "Formatted: {}",
        zoned.strftime("%A, %B %-d, %Y at %-I:%M %p %Z")
    );

    let date: Date = strtime::parse("%d/%m/%Y", "11/07/2024")?.to_date()?;
    println!("Custom:    {date}");

    let bad = "not a date".parse::<ZonedDateTime>();
    println!("Invalid:   {}", bad.is_err());

    Ok(())
}
