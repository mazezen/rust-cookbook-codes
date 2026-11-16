async fn fetch_network_request() -> u32 {
    89
}

#[tokio::main]
async fn main() {
    let result = fetch_network_request().await;
    assert_eq!(result, 89);
}
