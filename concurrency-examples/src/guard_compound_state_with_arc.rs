use anyhow::{Ok, Result, anyhow};
use std::sync::{Arc, Mutex};
use std::{assert_eq, println, thread};

#[derive(Debug, Default)]
struct Stats {
    count: u64,
    total: u64,
}

fn main() -> Result<()> {
    let stats = Arc::new(Mutex::new(Stats::default()));
    let mut handles = Vec::new();

    for value in 1..=10 {
        let stats = Arc::clone(&stats);
        let handle = thread::spawn(move || -> Result<()> {
            let mut stats = stats.lock().map_err(|_| anyhow!("mutex posionesd"))?;
            stats.count += 1;
            stats.total += value;
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))??;
    }

    let stats = stats.lock().map_err(|_| anyhow!("mutex poisoned"))?;
    println!("{:?}", *stats);
    assert_eq!(stats.count, 10);
    assert_eq!(stats.total, 55);

    Ok(())
}
