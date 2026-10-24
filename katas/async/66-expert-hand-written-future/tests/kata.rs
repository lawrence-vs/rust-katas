use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use kata_async_66::confirmation;
use tokio::task::yield_now;
use tokio::time::timeout;

const BOUND: Duration = Duration::from_secs(5);

#[tokio::test(start_paused = true)]
async fn a_parked_waiter_is_woken() {
    let (tx, rx) = confirmation();
    let handle = tokio::spawn(rx);
    yield_now().await; // the waiter is polled once and parks
    tx.confirm(42);
    let id = timeout(BOUND, handle)
        .await
        .expect("the waiter was never woken")
        .expect("the waiter task failed");
    assert_eq!(id, 42);
}

#[tokio::test(start_paused = true)]
async fn confirmed_before_await_is_ready() {
    let (tx, rx) = confirmation();
    tx.confirm(7);
    let id = timeout(BOUND, rx).await.expect("already confirmed");
    assert_eq!(id, 7);
}

#[tokio::test(start_paused = true)]
async fn the_latest_waker_is_woken() {
    let (tx, mut rx) = confirmation();
    let mut cx = Context::from_waker(Waker::noop());
    assert_eq!(Pin::new(&mut rx).poll(&mut cx), Poll::Pending);

    let handle = tokio::spawn(rx);
    yield_now().await;
    tx.confirm(9);
    let id = timeout(BOUND, handle)
        .await
        .expect("the latest waker was not woken")
        .expect("the waiter task failed");
    assert_eq!(id, 9);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn woken_from_another_thread() {
    let (tx, rx) = confirmation();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(20));
        tx.confirm(11);
    });
    let id = timeout(BOUND, rx)
        .await
        .expect("never woken across threads");
    assert_eq!(id, 11);
}
