use kata_capstone_215::{Book, BookError, Fill, OrderId, Side};
use std::time::{Duration, Instant};

fn fill(maker: u64, taker: u64, price: u64, qty: u64) -> Fill {
    Fill {
        maker: OrderId(maker),
        taker: OrderId(taker),
        price,
        qty,
    }
}

#[test]
fn stage1_cancel_head_middle_tail_keeps_fifo() {
    let mut book = Book::new();
    for i in 1..=5 {
        assert!(book.submit(OrderId(i), Side::Sell, 100, i).is_empty());
    }
    assert_eq!(book.cancel(OrderId(3)), Ok(3));
    assert_eq!(book.cancel(OrderId(1)), Ok(1));
    assert_eq!(book.cancel(OrderId(5)), Ok(5));
    assert_eq!(book.best_ask(), Some((100, 6)));
    assert_eq!(book.len(), 2);

    assert_eq!(
        book.submit(OrderId(9), Side::Buy, 100, 10),
        vec![fill(2, 9, 100, 2), fill(4, 9, 100, 4)]
    );
    assert_eq!(book.best_bid(), Some((100, 4)));
    assert_eq!(book.best_ask(), None);
    assert_eq!(book.len(), 1);
}

#[test]
fn stage2_freed_slot_never_cancels_a_stranger() {
    let mut book = Book::new();

    // Cancel, then the freed slot is reused by a new order.
    assert!(book.submit(OrderId(1), Side::Sell, 100, 5).is_empty());
    assert_eq!(book.cancel(OrderId(1)), Ok(5));
    assert!(book.submit(OrderId(2), Side::Sell, 100, 7).is_empty());
    assert_eq!(
        book.cancel(OrderId(1)),
        Err(BookError::UnknownOrder(OrderId(1)))
    );
    assert_eq!(book.best_ask(), Some((100, 7)));
    assert_eq!(book.len(), 1);
    assert_eq!(book.cancel(OrderId(2)), Ok(7));
    assert_eq!(book.best_ask(), None);
    assert_eq!(book.len(), 0);

    // Fill, then the freed slot is reused by a new order.
    assert!(book.submit(OrderId(10), Side::Sell, 101, 3).is_empty());
    assert_eq!(
        book.submit(OrderId(11), Side::Buy, 101, 3),
        vec![fill(10, 11, 101, 3)]
    );
    assert!(book.submit(OrderId(12), Side::Sell, 101, 4).is_empty());
    assert_eq!(
        book.cancel(OrderId(10)),
        Err(BookError::UnknownOrder(OrderId(10)))
    );
    assert_eq!(book.cancel(OrderId(12)), Ok(4));

    // Partial fill: the maker keeps resting, the taker never rests.
    assert!(book.submit(OrderId(20), Side::Sell, 102, 5).is_empty());
    assert_eq!(
        book.submit(OrderId(21), Side::Buy, 102, 2),
        vec![fill(20, 21, 102, 2)]
    );
    assert_eq!(book.cancel(OrderId(20)), Ok(3));
    assert!(book.cancel(OrderId(21)).is_err());
    assert_eq!(book.len(), 0);

    // An emptied level leaves the map.
    assert!(book.submit(OrderId(30), Side::Buy, 50, 1).is_empty());
    assert_eq!(book.cancel(OrderId(30)), Ok(1));
    assert_eq!(book.best_bid(), None);
}

#[test]
fn stage3_cancel_cost_does_not_grow_with_the_level() {
    const N: u64 = 100_000;
    const LIMIT: Duration = Duration::from_secs(2);
    let mut book = Book::new();
    for i in 1..=N {
        assert!(book.submit(OrderId(i), Side::Sell, 100, 1).is_empty());
    }
    assert_eq!(book.best_ask(), Some((100, N)));
    let start = Instant::now();
    for (done, i) in (1..=N).rev().enumerate() {
        assert_eq!(book.cancel(OrderId(i)), Ok(1));
        assert!(
            done % 1000 != 999 || start.elapsed() < LIMIT,
            "cancel is O(level): {:?} spent after {} cancels of {N}",
            start.elapsed(),
            done + 1
        );
    }
    assert!(
        start.elapsed() < LIMIT,
        "{:?} for {N} cancels",
        start.elapsed()
    );
    assert_eq!(book.len(), 0);
    assert_eq!(book.best_ask(), None);
}
