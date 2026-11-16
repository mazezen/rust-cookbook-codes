use std::println;
use tokio::signal;
use tokio::time::{Duration, sleep};

async fn fetch_data() -> String {
    // simulate a slow network request
    sleep(Duration::from_secs(5)).await;
    "data".to_string()
}

#[tokio::main]
async fn main() {
    loop {
        tokio::select! {
            result = fetch_data() => {
                println!("Got: {}", result);
            }
            _ = signal::ctrl_c() => {
                println!("Shutting down...");
                return;
            }
        }
    }
}
