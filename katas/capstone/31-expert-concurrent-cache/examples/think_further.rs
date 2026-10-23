//! 31 expert: capstone: a concurrent cache
//!
//! Think further, from the kata README:
//! 1. `load` can fail: say it returns `Result<V, DbError>`. Should a failed load be cached? With your stage 3 design, what do the nine waiting callers see when the first caller's load returns `Err`, and which `OnceCell` method changes that?
//! 2. The first caller's task is aborted (a `tokio::time::timeout` around `get_or_load`, or `JoinHandle::abort`) while its load is sleeping. What happens to the other nine, and how many times does `load` run in the end?
//! 3. Capacity is 2 and a third key arrives while a load for one of the first two is still running. Does an in-flight load count toward capacity? If it is evicted, what do its waiters get, and what stops the next caller from starting a second load for it?
//! 4. The values go stale (a fee tier, an instrument's tick size). How would you add a time-to-live, and how would you test it with paused time (`tokio::time::advance`)? At the moment a key expires, how many callers reload it?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::OnceCell;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // In the wild: ten callers ask for the same key; OnceCell runs the loader once and shares the result.
    let cell: Arc<OnceCell<u64>> = Arc::new(OnceCell::new());
    let loads = Arc::new(AtomicUsize::new(0));

    let mut callers = Vec::new();
    for _ in 0..10 {
        let cell = Arc::clone(&cell);
        let loads = Arc::clone(&loads);
        callers.push(tokio::spawn(async move {
            *cell
                .get_or_init(|| async {
                    loads.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                    99
                })
                .await
        }));
    }
    for c in callers {
        if let Ok(v) = c.await {
            println!("caller got {v}");
        }
    }
    println!("loader ran {} time(s)", loads.load(Ordering::SeqCst));

    // Think further 1: get_or_try_init lets a failed load be retried, instead of caching the error.
    let retry: OnceCell<u64> = OnceCell::new();
    let first: Result<&u64, &str> = retry.get_or_try_init(|| async { Err("db down") }).await;
    println!("first load: {first:?}");
    let second: Result<&u64, &str> = retry.get_or_try_init(|| async { Ok(7) }).await;
    println!("second load: {second:?}");
}
