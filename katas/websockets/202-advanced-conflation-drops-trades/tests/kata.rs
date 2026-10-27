use kata_websockets_202::{ClientBuffer, Event, Message, Price, Qty, Side, Symbol};

const BTC: Symbol = Symbol(1);
const ETH: Symbol = Symbol(2);

fn book(symbol: Symbol, side: Side, price: u64, qty: u64) -> Event {
    Event::Book {
        symbol,
        side,
        price: Price(price),
        qty: Qty(qty),
    }
}

fn trade(symbol: Symbol, price: u64, qty: u64) -> Event {
    Event::Trade {
        symbol,
        price: Price(price),
        qty: Qty(qty),
    }
}

fn ev(seq: u64, event: Event) -> Message {
    Message::Event { seq, event }
}

fn buffer(capacity: usize, events: &[(u64, Event)]) -> ClientBuffer {
    let mut b = ClientBuffer::new(capacity);
    for &(seq, event) in events {
        b.push(seq, event);
    }
    b
}

#[test]
fn trades_are_never_conflated() {
    let mut b = buffer(
        8,
        &[
            (1, trade(BTC, 100, 1)),
            (2, trade(BTC, 100, 1)),
            (3, book(BTC, Side::Bid, 100, 5)),
            (4, trade(BTC, 101, 2)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![
            ev(1, trade(BTC, 100, 1)),
            ev(2, trade(BTC, 100, 1)),
            ev(3, book(BTC, Side::Bid, 100, 5)),
            ev(4, trade(BTC, 101, 2)),
        ],
        "trades are never conflated"
    );
}

#[test]
fn book_updates_conflate_per_level() {
    let mut b = buffer(
        8,
        &[
            (1, book(BTC, Side::Bid, 100, 5)),
            (2, book(BTC, Side::Bid, 99, 2)),
            (3, book(BTC, Side::Ask, 100, 4)),
            (4, book(BTC, Side::Bid, 100, 7)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![
            ev(2, book(BTC, Side::Bid, 99, 2)),
            ev(3, book(BTC, Side::Ask, 100, 4)),
            ev(4, book(BTC, Side::Bid, 100, 7)),
        ],
        "book updates conflate per level"
    );
}

#[test]
fn symbols_conflate_separately() {
    let mut b = buffer(
        8,
        &[
            (1, book(BTC, Side::Bid, 100, 5)),
            (2, book(ETH, Side::Bid, 100, 6)),
            (3, book(BTC, Side::Bid, 100, 8)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![
            ev(2, book(ETH, Side::Bid, 100, 6)),
            ev(3, book(BTC, Side::Bid, 100, 8)),
        ],
        "symbols conflate separately"
    );
}

#[test]
fn an_update_to_a_queued_level_needs_no_new_slot() {
    let mut b = buffer(
        2,
        &[
            (1, trade(BTC, 100, 1)),
            (2, book(BTC, Side::Bid, 100, 5)),
            (3, book(BTC, Side::Bid, 100, 6)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![
            ev(1, trade(BTC, 100, 1)),
            ev(3, book(BTC, Side::Bid, 100, 6)),
        ],
        "an update to a queued level needs no new slot"
    );
}

#[test]
fn a_consumer_past_the_buffer_gets_a_resnapshot() {
    let mut b = buffer(
        2,
        &[
            (1, trade(BTC, 100, 1)),
            (2, trade(BTC, 100, 2)),
            (3, trade(BTC, 101, 1)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![Message::Resnapshot { seq: 3 }],
        "a consumer past the buffer gets a resnapshot"
    );
    b.push(4, book(BTC, Side::Bid, 100, 5));
    assert_eq!(
        b.drain(),
        vec![ev(4, book(BTC, Side::Bid, 100, 5))],
        "a consumer past the buffer gets a resnapshot"
    );
}

#[test]
fn events_after_a_resnapshot_keep_their_seq() {
    let mut b = buffer(
        3,
        &[
            (1, trade(BTC, 100, 1)),
            (2, trade(BTC, 100, 1)),
            (3, trade(BTC, 100, 1)),
            (4, trade(BTC, 100, 1)),
            (5, trade(ETH, 50, 2)),
            (6, book(BTC, Side::Ask, 101, 3)),
        ],
    );
    assert_eq!(
        b.drain(),
        vec![
            Message::Resnapshot { seq: 4 },
            ev(5, trade(ETH, 50, 2)),
            ev(6, book(BTC, Side::Ask, 101, 3)),
        ],
        "events after a resnapshot keep their seq"
    );
}

#[test]
fn drain_empties_the_buffer() {
    let mut b = buffer(4, &[(1, trade(BTC, 100, 1))]);
    assert_eq!(
        b.drain(),
        vec![ev(1, trade(BTC, 100, 1))],
        "drain empties the buffer"
    );
    assert_eq!(b.drain(), vec![], "drain empties the buffer");
}
