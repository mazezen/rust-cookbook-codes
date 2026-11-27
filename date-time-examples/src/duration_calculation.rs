use chrono::{DateTime, Duration as chronoDuration, FixedOffset, Local, Utc};
use std::println;
use std::time::{SystemTime, SystemTimeError};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn measure_the_elapsed_time_between_two_code() {
    let start = Instant::now();
    sleep(Duration::from_secs(1));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn deadline_arithmetic_with_instant() {
    let one_second = Duration::from_secs(1);
    let deadline = Instant::now() + one_second;

    sleep(Duration::from_secs(1));
    if Instant::now() > deadline {
        println!("deadline has passed!");
    }
}

fn conver_systemtime_to_unix_timestamp() -> Result<(), SystemTimeError> {
    let now = SystemTime::now();
    let since_unix_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;
    println!("Unix timestamp: {}", since_unix_epoch.as_secs());

    let from_unix_timestamp = SystemTime::UNIX_EPOCH + since_unix_epoch;
    println!("Back to SystemTime: {:?}", from_unix_timestamp);

    Ok(())
}

fn subset_sum(weights: &[u64], target: u64) -> Option<u32> {
    for mask in 0..1u32 << weights.len() {
        let sum: u64 = weights
            .iter()
            .enumerate()
            .filter(|(i, _)| mask >> i & 1 == 1)
            .map(|(_, weight)| weight)
            .sum();
        if sum == target {
            return Some(mask);
        }
    }
    None
}

fn benchmark<F, T>(runs: u32, mut f: F) -> Duration
where
    F: FnMut() -> T,
{
    let start = Instant::now();

    for _ in 0..runs {
        std::hint::black_box(f());
    }
    start.elapsed() / runs
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(chronoDuration::days(1))
}

fn main() {
    measure_the_elapsed_time_between_two_code();
    deadline_arithmetic_with_instant();
    conver_systemtime_to_unix_timestamp();

    let weights: Vec<u64> = (1..=18).map(|kg| kg * 2).collect();
    let half_load = weights.iter().sum::<u64>() / 2;

    for parcels in 12..=weights.len() {
        let mean = benchmark(3, || subset_sum(&weights[..parcels], half_load));
        println!("{:2} parcels: {:>10.2?}", parcels, mean);
    }

    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(chronoDuration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(chronoDuration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(chronoDuration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!(
            "We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."
        ),
    }

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );
}
