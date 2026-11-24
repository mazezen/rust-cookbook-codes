use anyhow::{Result, anyhow};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // One Sender clone per producer thread; all feed the single Receiver.
    let producers: Vec<_> = (0..3)
        .map(|id| {
            let tx = tx.clone();
            thread::spawn(move || -> Result<()> {
                for item in 0..3 {
                    tx.send((id, item))
                        .map_err(|e| anyhow!("send failed: {e}"))?;
                }
                Ok(())
            })
        })
        .collect();

    // Drop the original Sender so the channel closes once the clones do;
    // otherwise the loop below would block forever waiting for more senders.
    drop(tx);

    // Blocking, idiomatic consumption: iterate until every Sender is gone.
    let mut received = 0;
    for (id, item) in rx {
        println!("producer {id} sent {item}");
        received += 1;
    }

    for producer in producers {
        producer
            .join()
            .map_err(|_| anyhow!("producer panicked"))??;
    }
    assert_eq!(received, 9);
    Ok(())
}
