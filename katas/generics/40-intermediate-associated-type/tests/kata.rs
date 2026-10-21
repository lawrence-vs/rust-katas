use kata_generics_40::{Batcher, Feed, Trade, TradeFeed, drain};

fn trades() -> Vec<Trade> {
    vec![
        Trade { price: 100, qty: 2 },
        Trade { price: 101, qty: 1 },
        Trade { price: 99, qty: 5 },
    ]
}

/// Any feed of trades. The bound names the event type; there is no `T` to carry.
fn total_qty<F: Feed<Item = Trade>>(feed: &mut F) -> u64 {
    drain(feed).iter().map(|t| t.qty).sum()
}

/// Any feed at all. The event type comes from the feed.
fn first<F: Feed>(feed: &mut F) -> Option<F::Item> {
    feed.next_event()
}

#[test]
fn a_bound_names_the_item_type() {
    assert_eq!(total_qty(&mut TradeFeed::new(trades())), 8);
}

#[test]
fn the_item_type_comes_from_the_feed() {
    let mut feed = TradeFeed::new(trades());
    assert_eq!(first(&mut feed), Some(Trade { price: 100, qty: 2 }));
    assert_eq!(drain(&mut feed).len(), 2);
    assert_eq!(first(&mut feed), None);
}

#[test]
fn a_batcher_is_typed_by_its_feed_alone() {
    let mut batcher: Batcher<TradeFeed> = Batcher::new(TradeFeed::new(trades()), 2);
    assert_eq!(batcher.next_batch().map(|b| b.len()), Some(2));
    assert_eq!(
        batcher.next_batch(),
        Some(vec![Trade { price: 99, qty: 5 }])
    );
    assert_eq!(batcher.next_batch(), None);
}

#[test]
fn a_zero_batch_size_is_one() {
    let mut batcher: Batcher<TradeFeed> = Batcher::new(TradeFeed::new(trades()), 0);
    assert_eq!(batcher.next_batch().map(|b| b.len()), Some(1));
}
