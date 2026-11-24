use anyhow::{Result, anyhow};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || -> Result<()> {
        for command in ["load", "render", "save"] {
            tx.send(command).map_err(|e| anyhow!("send failed: {e}"))?;
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    });

    let mut frame = 0u32;
    loop {
        frame += 1;
        thread::sleep(Duration::from_millis(3));

        match rx.try_recv() {
            Ok(command) => println!("frame {frame}: handling {command}"),
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => break,
        }
    }

    producer
        .join()
        .map_err(|_| anyhow!("producer panicked"))??;
    println!("ran {frame} frames");
    Ok(())
}
