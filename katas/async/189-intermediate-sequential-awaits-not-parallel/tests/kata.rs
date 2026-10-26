use std::time::Duration;

use kata_async_189::quotes_from_two_venues;
use tokio::time::Instant;

#[tokio::test(start_paused = true)]
async fn quotes_run_in_parallel() {
    let start = Instant::now();
    assert_eq!(quotes_from_two_venues().await, (101, 102));
    assert!(
        start.elapsed() < Duration::from_millis(150),
        "took {:?}",
        start.elapsed()
    );
}
