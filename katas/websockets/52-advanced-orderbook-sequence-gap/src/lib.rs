use std::collections::BTreeMap;

/// A price in whole ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price(pub u64);

/// A quantity in whole lots. Zero means the level is gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qty(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

/// One level's new absolute quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    pub side: Side,
    pub price: Price,
    pub qty: Qty,
}

/// One message off the WebSocket feed. `seq` goes up by exactly 1 per message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta {
    pub seq: u64,
    pub changes: Vec<Change>,
}

/// The full book as of `seq` (it already contains every delta up to and including `seq`).
///
/// CLUE: A snapshot is the book *as of* `seq`. Which deltas does it already contain?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub seq: u64,
    pub bids: Vec<(Price, Qty)>,
    pub asks: Vec<(Price, Qty)>,
}

/// The REST endpoint that returns a full book.
pub trait SnapshotSource {
    fn snapshot(&mut self) -> Snapshot;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The delta was the next one and was applied.
    Applied,
    /// The delta was at or before the book's seq; nothing changed.
    Stale,
    /// The book was rebuilt from a snapshot at this seq (the delta was applied on top if it was the next one).
    Resynced { snapshot: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookError {
    /// The snapshot is older than the stream; the deltas in between are lost. The book is left empty and unsynced.
    SnapshotBehind { snapshot: u64, delta: u64 },
}

pub struct LocalBook<S> {
    source: S,
    /// CLUE: The last delta this book reflects. What does a delta whose seq isn't the next one tell you?
    seq: Option<u64>,
    bids: BTreeMap<Price, Qty>,
    asks: BTreeMap<Price, Qty>,
}

impl<S: SnapshotSource> LocalBook<S> {
    /// An empty, unsynced book. Fetches nothing.
    pub fn new(source: S) -> Self {
        LocalBook {
            source,
            seq: None,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn seq(&self) -> Option<u64> {
        self.seq
    }

    /// Levels best first: bids highest price first, asks lowest first; no zero quantities.
    pub fn levels(&self, side: Side) -> Vec<(Price, Qty)> {
        match side {
            Side::Bid => self.bids.iter().rev().map(|(p, q)| (*p, *q)).collect(),
            Side::Ask => self.asks.iter().map(|(p, q)| (*p, *q)).collect(),
        }
    }

    /// CLUE: A missed delta can't be rebuilt from the next one. What's the only source of truth left?
    ///
    /// CLUE: After a gap, which of these levels can you still trust?
    pub fn on_delta(&mut self, delta: Delta) -> Result<Outcome, BookError> {
        let mut outcome = Outcome::Applied;
        if self.seq.is_none() {
            let snapshot = self.source.snapshot();
            outcome = Outcome::Resynced {
                snapshot: snapshot.seq,
            };
            self.load(snapshot);
        }
        self.apply(&delta.changes);
        self.seq = Some(delta.seq);
        Ok(outcome)
    }

    fn load(&mut self, snapshot: Snapshot) {
        self.bids = snapshot
            .bids
            .into_iter()
            .filter(|(_, q)| q.0 != 0)
            .collect();
        self.asks = snapshot
            .asks
            .into_iter()
            .filter(|(_, q)| q.0 != 0)
            .collect();
        self.seq = Some(snapshot.seq);
    }

    fn apply(&mut self, changes: &[Change]) {
        for change in changes {
            let book = match change.side {
                Side::Bid => &mut self.bids,
                Side::Ask => &mut self.asks,
            };
            if change.qty.0 == 0 {
                book.remove(&change.price);
            } else {
                book.insert(change.price, change.qty);
            }
        }
    }
}
