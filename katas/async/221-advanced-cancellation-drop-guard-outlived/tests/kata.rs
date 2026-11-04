use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use kata_async_221::start;

async fn settle() {
    for _ in 0..5 {
        tokio::task::yield_now().await;
    }
}

#[tokio::test(start_paused = true)]
async fn worker_runs_while_owner_alive() {
    let ticks = Arc::new(AtomicUsize::new(0));
    let worker = start(ticks.clone());
    settle().await;
    tokio::time::advance(Duration::from_millis(100)).await;
    settle().await;
    assert!(
        ticks.load(Ordering::SeqCst) > 0,
        "worker stopped while its owner was alive"
    );
    drop(worker);
}

#[tokio::test(start_paused = true)]
async fn dropping_worker_stops_it() {
    let ticks = Arc::new(AtomicUsize::new(0));
    let worker = start(ticks.clone());
    settle().await;
    drop(worker);
    settle().await;
    let before = ticks.load(Ordering::SeqCst);
    tokio::time::advance(Duration::from_millis(100)).await;
    settle().await;
    assert_eq!(ticks.load(Ordering::SeqCst), before);
}
