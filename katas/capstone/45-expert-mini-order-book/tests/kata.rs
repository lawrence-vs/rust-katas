use kata_capstone_45::{Book, BookError, Fill, OrderId, Price, Side};

fn p(ticks: u64) -> Price {
    Price::from_ticks(ticks)
}

fn fill(maker: u64, taker: u64, price: u64, qty: u64) -> Fill {
    Fill {
        maker: OrderId(maker),
        taker: OrderId(taker),
        price: p(price),
        qty,
    }
}

#[test]
fn stage1_prices_are_whole_ticks() {
    for t in [1, 29, 115, 10_050] {
        assert_eq!(
            Price::from_ticks(t).ticks(),
            t,
            "{t} ticks came back different"
        );
    }
}

#[test]
fn stage2_best_price_first_then_oldest() {
    let mut book = Book::new();
    assert!(book.submit(OrderId(1), Side::Sell, p(101), 5).is_empty());
    assert!(book.submit(OrderId(2), Side::Sell, p(100), 5).is_empty());
    assert!(book.submit(OrderId(3), Side::Sell, p(100), 5).is_empty());

    assert!(
        book.submit(OrderId(4), Side::Buy, p(99), 1).is_empty(),
        "99 does not cross 100"
    );
    assert_eq!(book.best_bid(), Some((p(99), 1)));
    assert_eq!(book.best_ask(), Some((p(100), 10)));

    assert_eq!(
        book.submit(OrderId(5), Side::Buy, p(101), 1),
        vec![fill(2, 5, 100, 1)]
    );
    assert_eq!(
        book.submit(OrderId(6), Side::Buy, p(100), 4),
        vec![fill(2, 6, 100, 4)]
    );
    assert_eq!(
        book.submit(OrderId(7), Side::Buy, p(100), 1),
        vec![fill(3, 7, 100, 1)]
    );
}

#[test]
fn stage3_partial_fill_rests_the_remainder() {
    let mut book = Book::new();
    book.submit(OrderId(1), Side::Sell, p(99), 4);
    book.submit(OrderId(2), Side::Sell, p(100), 3);
    book.submit(OrderId(3), Side::Sell, p(101), 5);

    assert_eq!(
        book.submit(OrderId(4), Side::Buy, p(100), 10),
        vec![fill(1, 4, 99, 4), fill(2, 4, 100, 3)]
    );
    assert_eq!(book.best_bid(), Some((p(100), 3)));
    assert_eq!(book.best_ask(), Some((p(101), 5)));

    assert_eq!(
        book.submit(OrderId(5), Side::Sell, p(100), 1),
        vec![fill(4, 5, 100, 1)]
    );
    assert_eq!(book.best_bid(), Some((p(100), 2)));
}

#[test]
fn stage4_cancel_removes_the_order_and_its_empty_level() {
    let mut book = Book::new();
    book.submit(OrderId(1), Side::Sell, p(100), 2);
    book.submit(OrderId(2), Side::Sell, p(100), 3);
    book.submit(OrderId(3), Side::Sell, p(101), 4);

    assert_eq!(book.cancel(OrderId(1)), Ok(2));
    assert_eq!(book.best_ask(), Some((p(100), 3)));
    assert_eq!(
        book.cancel(OrderId(1)),
        Err(BookError::UnknownOrder(OrderId(1)))
    );
    assert_eq!(
        book.cancel(OrderId(99)),
        Err(BookError::UnknownOrder(OrderId(99)))
    );

    assert_eq!(book.cancel(OrderId(2)), Ok(3));
    assert_eq!(
        book.best_ask(),
        Some((p(101), 4)),
        "the empty 100 level must be gone"
    );

    assert_eq!(
        book.submit(OrderId(4), Side::Buy, p(101), 4),
        vec![fill(3, 4, 101, 4)]
    );
    assert_eq!(
        book.cancel(OrderId(3)),
        Err(BookError::UnknownOrder(OrderId(3)))
    );
    assert_eq!(book.best_ask(), None);
    assert_eq!(book.best_bid(), None);
}
