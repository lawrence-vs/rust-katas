use kata_rayon_59::*;
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

fn book(n: u32) -> Vec<Position> {
    (0..n)
        .map(|i| Position {
            id: PositionId(i),
            qty: u64::from(i) + 1,
        })
        .collect()
}

fn pool(threads: usize) -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("pool")
}

#[test]
fn valuations_come_back_in_input_order() {
    let book = book(1000);
    // Position 0 is slow: another worker steals the rest and finishes first.
    let out = pool(4).install(|| {
        value_book(&book, |p| {
            if p.id == PositionId(0) {
                std::thread::sleep(Duration::from_millis(500));
            }
            Cents(100)
        })
    });
    let ids: Vec<u32> = out.iter().map(|v| v.id.0).collect();
    let expected: Vec<u32> = (0..1000).collect();
    assert_eq!(ids, expected, "results come back in input order");
}

#[test]
fn each_valuation_is_qty_times_mark() {
    let book = book(3);
    let out = pool(1).install(|| value_book(&book, |p| Cents(u64::from(p.id.0) * 10 + 7)));
    assert_eq!(
        out,
        vec![
            Valuation {
                id: PositionId(0),
                notional: 7
            },
            Valuation {
                id: PositionId(1),
                notional: 34
            },
            Valuation {
                id: PositionId(2),
                notional: 81
            },
        ]
    );
}

#[test]
fn mark_is_called_once_per_position() {
    let book = book(1000);
    let calls = AtomicUsize::new(0);
    let out = value_book(&book, |_| {
        calls.fetch_add(1, Ordering::Relaxed);
        Cents(1)
    });
    assert_eq!(out.len(), 1000);
    assert_eq!(calls.load(Ordering::Relaxed), 1000);
}

#[test]
fn an_empty_book_values_to_nothing() {
    assert!(value_book(&[], |_| Cents(1)).is_empty());
    assert_eq!(total_exposure(&[], |_| Cents(1)), Some(0));
}

#[test]
fn exposure_sums_every_position() {
    let book = book(1000);
    let expected: u128 = 100 * (1..=1000u128).sum::<u128>();
    assert_eq!(total_exposure(&book, |_| Cents(100)), Some(expected));
}

#[test]
fn exposure_overflow_is_none() {
    let book = vec![
        Position {
            id: PositionId(0),
            qty: u64::MAX,
        },
        Position {
            id: PositionId(1),
            qty: u64::MAX,
        },
    ];
    assert_eq!(total_exposure(&book, |_| Cents(u64::MAX)), None);
}
