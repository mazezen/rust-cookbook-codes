use std::{eprintln, println, time::Duration};
use tokio::time::timeout;

async fn fetch_network_request() -> u32 {
    89
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(5), fetch_network_request()).await {
        Ok(x) => println!("Received {x}"),
        Err(_) => eprintln!("Timed out!"),
    }
}
