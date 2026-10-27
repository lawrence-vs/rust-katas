use kata_state_machines_201::{Book, BookError, Side};

/// Three buys at 100 with qty 5, seq 0, 1, 2.
fn book3() -> Book {
    let mut book = Book::new();
    assert_eq!(book.place(1, Side::Buy, 100, 5), Ok(0));
    assert_eq!(book.place(2, Side::Buy, 100, 5), Ok(1));
    assert_eq!(book.place(3, Side::Buy, 100, 5), Ok(2));
    book
}

#[test]
fn size_decrease_keeps_position() {
    let mut book = book3();
    assert_eq!(book.amend(1, 100, 3), Ok(0));
    assert_eq!(
        book.queue(Side::Buy, 100),
        [(1, 3, 0), (2, 5, 1), (3, 5, 2)]
    );
}

#[test]
fn size_increase_loses_queue_position() {
    let mut book = book3();
    assert_eq!(book.amend(1, 100, 8), Ok(3));
    assert_eq!(
        book.queue(Side::Buy, 100),
        [(2, 5, 1), (3, 5, 2), (1, 8, 3)]
    );
}

#[test]
fn price_change_joins_back_of_new_level() {
    let mut book = Book::new();
    assert_eq!(book.place(1, Side::Buy, 100, 5), Ok(0));
    assert_eq!(book.place(2, Side::Buy, 100, 5), Ok(1));
    assert_eq!(book.place(3, Side::Buy, 99, 5), Ok(2));
    assert_eq!(book.place(4, Side::Buy, 99, 5), Ok(3));

    assert_eq!(book.amend(2, 99, 5), Ok(4));
    assert_eq!(book.queue(Side::Buy, 99), [(3, 5, 2), (4, 5, 3), (2, 5, 4)]);
    assert_eq!(book.queue(Side::Buy, 100), [(1, 5, 0)]);

    assert_eq!(book.amend(1, 98, 5), Ok(5));
    assert_eq!(book.best_bid(), Some(99));
    assert!(book.queue(Side::Buy, 100).is_empty());
}

#[test]
fn fifo_holds_after_amends() {
    let mut book = book3();
    assert_eq!(book.amend(2, 100, 2), Ok(1));
    assert_eq!(book.amend(1, 100, 9), Ok(3));
    assert_eq!(book.amend(3, 100, 6), Ok(4));

    let queue = book.queue(Side::Buy, 100);
    assert_eq!(queue, [(2, 2, 1), (1, 9, 3), (3, 6, 4)]);
    assert!(queue.windows(2).all(|w| w[0].2 < w[1].2));
}

#[test]
fn book_is_never_crossed() {
    let mut book = Book::new();
    assert_eq!(book.place(1, Side::Buy, 100, 5), Ok(0));
    assert_eq!(book.place(2, Side::Sell, 101, 5), Ok(1));

    assert_eq!(book.place(3, Side::Buy, 101, 1), Err(BookError::WouldCross));
    assert_eq!(
        book.place(4, Side::Sell, 100, 1),
        Err(BookError::WouldCross)
    );
    assert_eq!(book.amend(1, 101, 5), Err(BookError::WouldCross));
    assert_eq!(book.queue(Side::Buy, 100), [(1, 5, 0)]);

    assert_eq!(book.place(5, Side::Buy, 99, 1), Ok(2));
    assert!(book.best_bid() < book.best_ask());
}

#[test]
fn bad_commands_are_errors() {
    let mut book = Book::new();
    assert_eq!(book.amend(99, 100, 1), Err(BookError::UnknownId));

    assert_eq!(book.place(1, Side::Buy, 100, 5), Ok(0));
    assert_eq!(book.amend(1, 100, 0), Err(BookError::ZeroQty));
    assert_eq!(
        book.place(1, Side::Buy, 100, 5),
        Err(BookError::DuplicateId)
    );
    assert_eq!(book.place(7, Side::Buy, 50, 0), Err(BookError::ZeroQty));
    assert_eq!(book.cancel(99), Err(BookError::UnknownId));

    assert_eq!(book.cancel(1), Ok(()));
    assert_eq!(book.amend(1, 100, 1), Err(BookError::UnknownId));
    assert_eq!(book.best_bid(), None);
}
