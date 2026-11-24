use anyhow::{Ok, Result, anyhow};
use std::sync::{Arc, RwLock};
use std::{println, thread};

fn main() -> Result<()> {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = Vec::new();

    for id in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || -> Result<()> {
            let reader = data.read().map_err(|_| anyhow!("lock poisoned"))?;
            println!("reader {} sees {:?}", id, *reader);
            Ok(())
        }));
    }
    for handle in handles {
        handle.join().map_err(|_| anyhow!("thread panicked"))??;
    }

    let final_state = data.read().map_err(|_| anyhow!("lock poisoned"))?;
    println!("final: {:?}", *final_state);
    Ok(())
}
