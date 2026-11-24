use anyhow::{Result, anyhow};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{assert_eq, println, thread};

fn main() -> Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Relaxed is enough: we only care that the count is exact.
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))?;
    }

    // join() above synchronizes the threads, so a Relaxed load sees the total.
    let total = counter.load(Ordering::Relaxed);
    println!("total: {}", total);
    assert_eq!(total, 10_000);
    Ok(())
}
