//! A feed of JSON order-book updates: parse, rebuild per-symbol books, recompute depth and VWAP
//! for the touched symbols in parallel, publish. Four things are wrong; the tests find them in order.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::num::NonZeroUsize;
use std::sync::Arc;

use rayon::prelude::*;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// The highest price, in ticks.
pub const MAX_PRICE: u64 = 1_000_000_000_000;
/// The highest quantity, in lots.
pub const MAX_QTY: u64 = 1_000_000_000_000;

/// A price in whole ticks, `1..=MAX_PRICE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price(pub u64);

/// A quantity in whole lots, `0..=MAX_QTY`. Zero removes the level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Qty(pub u64);

/// An instrument name: 1 to 16 characters of `A-Z`, `0-9` and `-`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Symbol {
    type Error = FeedError;

    fn try_from(s: String) -> Result<Self, FeedError> {
        Ok(Symbol(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub symbol: Symbol,
    pub seq: u64,
    pub side: Side,
    pub price: Price,
    pub qty: Qty,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FeedError {
    #[error("invalid update: {0}")]
    Invalid(String),
    #[error("the feed is closed")]
    Closed,
    #[error("the pipeline failed: {0}")]
    Pipeline(String),
}

/// Parse one JSON line:
/// `{"symbol":"BTC-USD","seq":7,"side":"bid","price":100,"qty":5}`.
/// All five fields are required, unknown fields are rejected, `price` and `qty` are integers.
///
/// CLUE: what does `as u64` do to `-5`, `1.5` and a missing field? Look at serde's
/// `deny_unknown_fields` and at `TryFrom`.
pub fn parse_update(line: &str) -> Result<Update, FeedError> {
    let v: Value = serde_json::from_str(line).unwrap_or_default();
    let side = if v["side"].as_str() == Some("ask") {
        Side::Ask
    } else {
        Side::Bid
    };
    Ok(Update {
        symbol: Symbol(v["symbol"].as_str().unwrap_or_default().to_string()),
        seq: v["seq"].as_u64().unwrap_or_default(),
        side,
        price: Price(v["price"].as_f64().unwrap_or_default() as u64),
        qty: Qty(v["qty"].as_f64().unwrap_or_default() as u64),
    })
}

/// One symbol's book: the last applied `seq` and the resting quantity at each price.
#[derive(Debug, Clone, Default)]
pub struct Book {
    seq: u64,
    bids: BTreeMap<Price, Qty>,
    asks: BTreeMap<Price, Qty>,
}

impl Book {
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Best first: bids high to low, asks low to high.
    pub fn levels(&self, side: Side) -> Vec<(Price, Qty)> {
        match side {
            Side::Bid => self.bids.iter().rev().map(|(p, q)| (*p, *q)).collect(),
            Side::Ask => self.asks.iter().map(|(p, q)| (*p, *q)).collect(),
        }
    }

    fn apply(&mut self, u: &Update) {
        self.seq = u.seq;
        let side = match u.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };
        if u.qty.0 == 0 {
            side.remove(&u.price);
        } else {
            side.insert(u.price, u.qty);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideDepth {
    pub qty: Qty,
    pub vwap: Option<Price>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depth {
    pub symbol: Symbol,
    pub seq: u64,
    pub bid: SideDepth,
    pub ask: SideDepth,
}

/// Computes a symbol's depth from its book. Runs on a rayon thread.
pub trait Pricer: Send + Sync + 'static {
    fn depth(&self, symbol: &Symbol, book: &Book) -> Depth;
}

/// Depth and VWAP over the best `n` levels per side. VWAP rounds down to a whole tick and is
/// `None` when the side is empty.
pub struct TopLevels(pub usize);

impl TopLevels {
    fn side(&self, levels: Vec<(Price, Qty)>) -> SideDepth {
        let (mut qty, mut notional) = (0u128, 0u128);
        for (p, q) in levels.into_iter().take(self.0) {
            qty += u128::from(q.0);
            notional += u128::from(p.0) * u128::from(q.0);
        }
        SideDepth {
            qty: Qty(u64::try_from(qty).unwrap_or(u64::MAX)),
            vwap: notional
                .checked_div(qty)
                .map(|v| Price(u64::try_from(v).unwrap_or(u64::MAX))),
        }
    }
}

impl Pricer for TopLevels {
    fn depth(&self, symbol: &Symbol, book: &Book) -> Depth {
        Depth {
            symbol: symbol.clone(),
            seq: book.seq(),
            bid: self.side(book.levels(Side::Bid)),
            ask: self.side(book.levels(Side::Ask)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// How many updates may wait between `submit` and the pipeline.
    pub capacity: NonZeroUsize,
}

pub struct Feed {
    tx: mpsc::UnboundedSender<Update>,
    task: JoinHandle<()>,
}

impl Feed {
    /// Must be called inside a tokio runtime.
    ///
    /// CLUE: `config.capacity` is never read. What happens to memory when the producer outruns
    /// the pipeline?
    pub fn start<P: Pricer, F: FnMut(Depth) + Send + 'static>(
        _config: Config,
        pricer: P,
        publish: F,
    ) -> Feed {
        let (tx, rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(run(rx, Arc::new(pricer), publish));
        Feed { tx, task }
    }

    pub async fn submit(&self, line: &str) -> Result<(), FeedError> {
        let u = parse_update(line)?;
        self.tx.send(u).map_err(|_| FeedError::Closed)
    }

    /// Stop the feed. Every update accepted by `submit` must be published before this returns.
    ///
    /// CLUE: what happens to updates still in the queue when the task is aborted?
    pub async fn shutdown(self) -> Result<(), FeedError> {
        self.task.abort();
        Ok(())
    }
}

/// CLUE: which thread waits while `par_iter` collects, and what else wanted that thread?
async fn run<P: Pricer, F: FnMut(Depth)>(
    mut rx: mpsc::UnboundedReceiver<Update>,
    pricer: Arc<P>,
    mut publish: F,
) {
    let mut books: HashMap<Symbol, Book> = HashMap::new();
    while let Some(first) = rx.recv().await {
        let mut touched: HashSet<Symbol> = HashSet::new();
        let mut apply = |u: Update| {
            books.entry(u.symbol.clone()).or_default().apply(&u);
            touched.insert(u.symbol);
        };
        apply(first);
        while let Ok(u) = rx.try_recv() {
            apply(u);
        }
        let depths: Vec<Depth> = touched
            .par_iter()
            .map(|s| pricer.depth(s, &books[s]))
            .collect();
        for d in depths {
            publish(d);
        }
    }
}
