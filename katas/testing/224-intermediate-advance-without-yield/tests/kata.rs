use kata_testing_224::advance_until;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn a_task_that_needs_several_turns_after_its_timer_fires() {
    let flag = Arc::new(AtomicBool::new(false));
    let f = flag.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        for _ in 0..8 {
            tokio::task::yield_now().await;
        }
        f.store(true, Ordering::SeqCst);
    });
    assert!(
        advance_until(Duration::from_millis(100), 5, || flag
            .load(Ordering::SeqCst))
        .await,
        "condition never met"
    );
}

#[tokio::test(start_paused = true)]
async fn already_done_returns_immediately() {
    assert!(advance_until(Duration::from_millis(10), 1, || true).await);
}
