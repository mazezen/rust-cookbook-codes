use std::io;
use tokio::runtime::Builder;

async fn fetch_network_request() -> u32 {
    89
}

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("thread-one")
        .thread_stack_size(3 * 1024 * 1024)
        .build()?;

    runtime.spawn(async {
        let result = fetch_network_request().await;
        assert_eq!(result, 89);
    });

    Ok(())
}
