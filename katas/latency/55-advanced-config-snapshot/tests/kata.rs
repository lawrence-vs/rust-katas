use kata_latency_55::{Cents, ConfigStore, Quote, QuoteConfig, QuoteError, Quoter, StoreError};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

struct Inner {
    config: Mutex<QuoteConfig>,
    loads: AtomicUsize,
    down: AtomicBool,
}

#[derive(Clone)]
struct FakeStore {
    inner: Arc<Inner>,
}

impl FakeStore {
    fn new(config: QuoteConfig) -> Self {
        Self {
            inner: Arc::new(Inner {
                config: Mutex::new(config),
                loads: AtomicUsize::new(0),
                down: AtomicBool::new(false),
            }),
        }
    }

    fn set(&self, config: QuoteConfig) {
        *self.inner.config.lock().unwrap() = config;
    }

    fn set_down(&self, down: bool) {
        self.inner.down.store(down, Ordering::SeqCst);
    }

    fn loads(&self) -> usize {
        self.inner.loads.load(Ordering::SeqCst)
    }
}

impl ConfigStore for FakeStore {
    fn load(&self) -> Result<QuoteConfig, StoreError> {
        self.inner.loads.fetch_add(1, Ordering::SeqCst);
        if self.inner.down.load(Ordering::SeqCst) {
            return Err(StoreError);
        }
        Ok(self.inner.config.lock().unwrap().clone())
    }
}

fn config(version: u64, half_spread: u64) -> QuoteConfig {
    QuoteConfig {
        version,
        half_spread: Cents(half_spread),
        max_size: 100,
    }
}

const MID: Cents = Cents(10_000);

#[test]
fn quotes_use_the_loaded_config() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store).unwrap();
    assert_eq!(
        quoter.quote(MID, 10),
        Ok(Quote {
            bid: Cents(9_995),
            ask: Cents(10_005),
            version: 1
        })
    );
}

#[test]
fn a_quote_over_max_size_is_refused() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store).unwrap();
    assert_eq!(
        quoter.quote(MID, 101),
        Err(QuoteError::TooLarge {
            requested: 101,
            max: 100
        })
    );
}

#[test]
fn quoting_does_not_read_the_store() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store.clone()).unwrap();
    for _ in 0..1_000 {
        assert!(quoter.quote(MID, 10).is_ok());
    }
    assert_eq!(store.loads(), 1, "a quote read the config store");
}

#[test]
fn a_change_is_seen_only_after_reload() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store.clone()).unwrap();
    store.set(config(2, 7));
    assert_eq!(
        quoter.quote(MID, 10).map(|q| q.version),
        Ok(1),
        "a quote saw a config that was never loaded"
    );
    assert_eq!(quoter.reload(), Ok(2));
    assert_eq!(
        quoter.quote(MID, 10),
        Ok(Quote {
            bid: Cents(9_993),
            ask: Cents(10_007),
            version: 2
        })
    );
}

#[test]
fn a_failed_reload_keeps_the_last_good_config() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store.clone()).unwrap();
    store.set(config(2, 7));
    store.set_down(true);
    assert_eq!(quoter.reload(), Err(StoreError));
    assert_eq!(
        quoter.quote(MID, 10).map(|q| q.version),
        Ok(1),
        "a failed reload keeps the last good config"
    );
    store.set_down(false);
    assert_eq!(quoter.reload(), Ok(2));
}

#[test]
fn one_quoter_serves_many_threads_through_a_reload() {
    let store = FakeStore::new(config(1, 5));
    let quoter = Quoter::new(store.clone()).unwrap();
    store.set(config(2, 7));
    let quotes: Vec<Quote> = thread::scope(|s| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let quoter = &quoter;
                s.spawn(move || {
                    (0..250)
                        .filter_map(|_| quoter.quote(MID, 10).ok())
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        assert_eq!(quoter.reload(), Ok(2));
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });
    assert_eq!(quotes.len(), 1_000);
    for q in quotes {
        match q.version {
            1 => assert_eq!((q.bid, q.ask), (Cents(9_995), Cents(10_005))),
            2 => assert_eq!((q.bid, q.ask), (Cents(9_993), Cents(10_007))),
            v => panic!("a quote used unknown version {v}"),
        }
    }
    assert_eq!(store.loads(), 2, "a quote read the config store");
}
