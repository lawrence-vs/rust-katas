use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kata_capstone_60::*;
use tokio::sync::Notify;
use tokio::time::timeout;

fn line(sym: &str, seq: u64, side: &str, price: u64, qty: u64) -> String {
    format!(r#"{{"symbol":"{sym}","seq":{seq},"side":"{side}","price":{price},"qty":{qty}}}"#)
}

fn config(capacity: usize) -> Config {
    Config {
        capacity: NonZeroUsize::new(capacity).unwrap(),
    }
}

type Published = Arc<Mutex<Vec<Depth>>>;

fn publisher() -> (Published, impl FnMut(Depth) + Send + 'static) {
    let out: Published = Arc::default();
    let sink = Arc::clone(&out);
    (out, move |d| sink.lock().unwrap().push(d))
}

fn last_for(out: &Published, sym: &str) -> Option<Depth> {
    out.lock()
        .unwrap()
        .iter()
        .rev()
        .find(|d| d.symbol.as_str() == sym)
        .cloned()
}

/// Holds the first recompute (on whatever thread runs it) until the test releases it.
/// If the wait times out, whatever the test needed to run on was busy: `blocked` is set.
struct Gate {
    inner: TopLevels,
    started: Arc<Notify>,
    first: Mutex<Option<std::sync::mpsc::Receiver<()>>>,
    blocked: Arc<AtomicBool>,
    wait: Duration,
}

struct Handles {
    release: std::sync::mpsc::Sender<()>,
    started: Arc<Notify>,
    blocked: Arc<AtomicBool>,
}

fn gate(n: usize, wait: Duration) -> (Gate, Handles) {
    let (release, rx) = std::sync::mpsc::channel();
    let started = Arc::new(Notify::new());
    let blocked = Arc::new(AtomicBool::new(false));
    let gate = Gate {
        inner: TopLevels(n),
        started: Arc::clone(&started),
        first: Mutex::new(Some(rx)),
        blocked: Arc::clone(&blocked),
        wait,
    };
    (
        gate,
        Handles {
            release,
            started,
            blocked,
        },
    )
}

impl Pricer for Gate {
    fn depth(&self, symbol: &Symbol, book: &Book) -> Depth {
        // The guard is held while waiting, so any other recompute queues up behind the first.
        let mut first = self.first.lock().unwrap();
        if let Some(rx) = first.take() {
            self.started.notify_one();
            if rx.recv_timeout(self.wait).is_err() {
                self.blocked.store(true, Ordering::SeqCst);
            }
        }
        self.inner.depth(symbol, book)
    }
}

#[tokio::test]
async fn stage1_a_valid_update_parses() {
    let u = parse_update(&line("BTC-USD", 7, "bid", 100, 5)).unwrap();
    assert_eq!(u.symbol.as_str(), "BTC-USD");
    assert_eq!(u.seq, 7);
    assert_eq!(u.side, Side::Bid);
    assert_eq!(u.price, Price(100));
    assert_eq!(u.qty, Qty(5));

    let removal = parse_update(&line("BTC-USD", 8, "ask", 100, 0)).unwrap();
    assert_eq!(removal.side, Side::Ask);
    assert_eq!(removal.qty, Qty(0));
}

#[tokio::test]
async fn stage1_invalid_updates_are_rejected() {
    let too_dear = MAX_PRICE + 1;
    let too_many = MAX_QTY + 1;
    let bad = [
        "{not json".to_string(),
        r#"{"symbol":"BTC-USD","seq":1,"side":"bid","price":-5,"qty":5}"#.to_string(),
        r#"{"symbol":"BTC-USD","seq":1,"side":"bid","price":1.5,"qty":5}"#.to_string(),
        r#"{"symbol":"BTC-USD","seq":1,"side":"bid","price":"100","qty":5}"#.to_string(),
        line("BTC-USD", 1, "bid", 0, 5),
        line("BTC-USD", 1, "bid", too_dear, 5),
        line("BTC-USD", 1, "bid", 100, too_many),
        r#"{"symbol":"BTC-USD","seq":1,"side":"bid","price":100}"#.to_string(),
        r#"{"symbol":"BTC-USD","seq":1,"side":"bid","price":100,"qty":5,"note":"x"}"#.to_string(),
        line("BTC-USD", 1, "buy", 100, 5),
        line("", 1, "bid", 100, 5),
        line("btc usd", 1, "bid", 100, 5),
    ];
    for l in &bad {
        assert!(
            matches!(parse_update(l), Err(FeedError::Invalid(_))),
            "an invalid update was accepted: {l}"
        );
    }
}

#[tokio::test]
async fn stage2_the_recompute_does_not_block_the_runtime() {
    let (gate, h) = gate(3, Duration::from_secs(2));
    let (_out, publish) = publisher();
    let feed = Feed::start(config(8), gate, publish);

    feed.submit(&line("BTC-USD", 1, "bid", 100, 5))
        .await
        .unwrap();
    h.started.notified().await;
    let _ = h.release.send(());

    let done = timeout(Duration::from_secs(5), feed.shutdown()).await;
    assert!(
        matches!(done, Ok(Ok(()))),
        "shutdown did not finish: {done:?}"
    );
    assert!(
        !h.blocked.load(Ordering::SeqCst),
        "the recompute blocked the runtime"
    );
}

#[tokio::test]
async fn stage3_a_full_queue_pushes_back() {
    let (gate, h) = gate(3, Duration::from_secs(2));
    let (out, publish) = publisher();
    let feed = Feed::start(config(2), gate, publish);
    let sym = "BTC-USD";

    feed.submit(&line(sym, 1, "bid", 100, 1)).await.unwrap();
    h.started.notified().await;

    for seq in [2, 3] {
        let sent = timeout(
            Duration::from_secs(1),
            feed.submit(&line(sym, seq, "bid", 100 - seq, 1)),
        )
        .await;
        assert!(
            matches!(sent, Ok(Ok(()))),
            "seq {seq} was not accepted: {sent:?}"
        );
    }

    let fourth = line(sym, 4, "bid", 96, 1);
    assert!(
        timeout(Duration::from_millis(200), feed.submit(&fourth))
            .await
            .is_err(),
        "a full queue did not push back"
    );

    let _ = h.release.send(());
    let sent = timeout(Duration::from_secs(1), feed.submit(&fourth)).await;
    assert!(
        matches!(sent, Ok(Ok(()))),
        "seq 4 was not accepted: {sent:?}"
    );

    let done = timeout(Duration::from_secs(5), feed.shutdown()).await;
    assert!(
        matches!(done, Ok(Ok(()))),
        "shutdown did not finish: {done:?}"
    );
    assert_eq!(last_for(&out, sym).map(|d| d.seq), Some(4));
}

#[tokio::test]
async fn stage4_shutdown_publishes_every_update() {
    let (out, publish) = publisher();
    let feed = Feed::start(config(4), TopLevels(3), publish);
    let symbols = ["BTC-USD", "ETH-USD", "SOL-USD"];

    for seq in 1..=10u64 {
        for sym in symbols {
            let l = if seq % 2 == 1 {
                line(sym, seq, "bid", 100 - seq, seq)
            } else {
                line(sym, seq, "ask", 100 + seq, seq)
            };
            feed.submit(&l).await.unwrap();
        }
    }
    let done = timeout(Duration::from_secs(5), feed.shutdown()).await;
    assert!(
        matches!(done, Ok(Ok(()))),
        "shutdown did not finish: {done:?}"
    );

    // Bids 99x1 + 97x3 + 95x5 = 865, and 865 / 9 = 96 (rounded down).
    // Asks 102x2 + 104x4 + 106x6 = 1256, and 1256 / 12 = 104 (rounded down).
    for sym in symbols {
        let want = Depth {
            symbol: Symbol::try_from(sym.to_string()).unwrap(),
            seq: 10,
            bid: SideDepth {
                qty: Qty(9),
                vwap: Some(Price(96)),
            },
            ask: SideDepth {
                qty: Qty(12),
                vwap: Some(Price(104)),
            },
        };
        assert_eq!(
            last_for(&out, sym),
            Some(want),
            "updates were lost on shutdown"
        );
    }
}
