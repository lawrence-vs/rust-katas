//! A market-data feed: something that hands out events one at a time.

use std::marker::PhantomData;

/// One fill on the tape. Prices are whole ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trade {
    pub price: u64,
    pub qty: u64,
}

/// A source of events, read one at a time.
/// CLUE: Could `TradeFeed` sensibly hand out anything but `Trade`? Who should choose the event type: the caller, or the feed?
/// CLUE: The tests write `Feed<Item = Trade>` and `F::Item`. Which kind of trait item lets a bound name a trait's output like that?
pub trait Feed<T> {
    fn next_event(&mut self) -> Option<T>;
}

/// Replays a fixed list of trades.
pub struct TradeFeed {
    trades: std::vec::IntoIter<Trade>,
}

impl TradeFeed {
    pub fn new(trades: Vec<Trade>) -> Self {
        Self {
            trades: trades.into_iter(),
        }
    }
}

impl Feed<Trade> for TradeFeed {
    fn next_event(&mut self) -> Option<Trade> {
        self.trades.next()
    }
}

/// Reads everything left in `feed`.
pub fn drain<T, F: Feed<T>>(feed: &mut F) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(event) = feed.next_event() {
        out.push(event);
    }
    out
}

/// Hands out a feed's events in batches of `size`.
/// CLUE: What is `PhantomData<T>` here for? Would `Batcher` need `T` at all if the feed named its own event type?
pub struct Batcher<F, T> {
    feed: F,
    size: usize,
    _item: PhantomData<T>,
}

impl<T, F: Feed<T>> Batcher<F, T> {
    /// A `size` of 0 is treated as 1.
    pub fn new(feed: F, size: usize) -> Self {
        Self {
            feed,
            size: size.max(1),
            _item: PhantomData,
        }
    }

    /// The next batch, or `None` once the feed is empty.
    pub fn next_batch(&mut self) -> Option<Vec<T>> {
        let batch: Vec<T> = (0..self.size)
            .map_while(|_| self.feed.next_event())
            .collect();
        if batch.is_empty() { None } else { Some(batch) }
    }
}
