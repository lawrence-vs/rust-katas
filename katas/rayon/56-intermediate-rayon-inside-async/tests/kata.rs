use kata_rayon_56::{Level, PRICING_TIME, book_value};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn book() -> Vec<Level> {
    vec![
        Level {
            price: 100,
            size: 3,
        },
        Level {
            price: 101,
            size: 2,
        },
    ]
}

#[tokio::test]
async fn the_timer_keeps_ticking_while_the_book_is_valued() {
    let ticks = Arc::new(Mutex::new(Vec::new()));
    let heartbeat = tokio::spawn({
        let ticks = Arc::clone(&ticks);
        async move {
            loop {
                ticks.lock().unwrap().push(Instant::now());
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        }
    });
    tokio::task::yield_now().await; // the heartbeat takes its first tick
    let value = book_value(book()).await;
    heartbeat.abort();
    let mut ticks = ticks.lock().unwrap().clone();
    ticks.push(Instant::now()); // the gap up to the end counts too
    let longest = ticks.windows(2).map(|w| w[1] - w[0]).max().unwrap();
    assert_eq!(value, Ok(502));
    assert!(
        longest < PRICING_TIME / 3,
        "the timer task stalled while the book was valued: {longest:?}"
    );
}

#[tokio::test]
async fn levels_are_still_valued_in_parallel() {
    let start = Instant::now();
    assert_eq!(book_value(book()).await, Ok(502));
    let took = start.elapsed();
    assert!(
        took < PRICING_TIME * 5 / 3,
        "levels were valued one after another: {took:?}"
    );
}
