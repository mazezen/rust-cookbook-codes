use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    /// When we should resolve.
    deadline: Instant,
}

impl Delay {
    fn new(dur: Duration) -> Self {
        Self {
            deadline: Instant::now() + dur,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.deadline {
            // Deadline reached — resolve the future.
            Poll::Ready(())
        } else {
            // Not ready yet.
            //
            // We MUST arrange for `wake()` to be called later or the
            // executor will never poll us again.  A production timer
            // would register with a reactor; here we just ask to be
            // re-polled immediately. The executor may yield to other
            // tasks before coming back to us.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    println!("waiting 10 ms …");

    // Use the custom future just like any other async expression.
    Delay::new(Duration::from_millis(10)).await;

    println!("done in {:?}", start.elapsed());
    assert!(start.elapsed() >= Duration::from_millis(10));
}
