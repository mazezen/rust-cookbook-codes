use anyhow::{Ok, Result, anyhow};
use std::sync::{Arc, Barrier};
use std::{println, thread};

fn main() -> Result<()> {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();

    for id in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            println!("thread {} starting phase 2", id);
        }));
    }

    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))?;
    }
    Ok(())
}
