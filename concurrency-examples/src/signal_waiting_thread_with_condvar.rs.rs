use anyhow::{Ok, Result, anyhow};
use std::sync::{Arc, Condvar, Mutex};
use std::{println, thread};

fn main() -> Result<()> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let signaler = Arc::clone(&pair);
    let worker = thread::spawn(move || -> Result<()> {
        let (lock, cvar) = &*signaler;
        let mut ready = lock.lock().map_err(|_| anyhow!("mutex posioned"))?;
        *ready = true;

        drop(ready);
        cvar.notify_one();
        Ok(())
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().map_err(|_| anyhow!("mutex posioned"))?;
    while !*ready {
        ready = cvar.wait(ready).map_err(|_| anyhow!("mutex posioned"))?;
    }
    println!("worker signaler readiness");
    worker.join().map_err(|_| anyhow!("worker panicked"))??;
    Ok(())
}
