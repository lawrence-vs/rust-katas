//! Quotes a bid and ask from a mid and the desk's quote config.

/// Quote-currency cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cents(pub u64);

/// The desk's quoting settings, as stored in the config database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteConfig {
    /// Goes up by one on every change.
    pub version: u64,
    /// Taken off the mid for the bid, added to it for the ask.
    pub half_spread: Cents,
    /// The largest size the desk quotes.
    pub max_size: u64,
}

/// The config database could not be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreError;

/// Where the config lives. Every `load` is a round trip to the database.
pub trait ConfigStore {
    fn load(&self) -> Result<QuoteConfig, StoreError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quote {
    pub bid: Cents,
    pub ask: Cents,
    /// The config version this quote was priced with.
    pub version: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteError {
    TooLarge {
        requested: u64,
        max: u64,
    },
    /// The bid or ask is outside `u64` cents.
    OutOfRange,
    Store(StoreError),
}

impl From<StoreError> for QuoteError {
    fn from(e: StoreError) -> Self {
        QuoteError::Store(e)
    }
}

/// CLUE: Every quote pays a store round trip here. What does a quote need: the store, or the last config the store returned?
pub struct Quoter<S> {
    store: S,
}

impl<S: ConfigStore> Quoter<S> {
    /// Fails if the store can't be read at startup.
    pub fn new(store: S) -> Result<Self, StoreError> {
        store.load()?;
        Ok(Self { store })
    }

    /// CLUE: If the database is down for a second, should quoting stop? Which error variant exists only because a quote asks the store?
    /// CLUE: How long should a quote hold a lock: for the whole price, or just long enough to take its own handle on the config? (kata 18)
    pub fn quote(&self, mid: Cents, size: u64) -> Result<Quote, QuoteError> {
        let config = self.store.load()?;
        if size > config.max_size {
            return Err(QuoteError::TooLarge {
                requested: size,
                max: config.max_size,
            });
        }
        let bid = mid
            .0
            .checked_sub(config.half_spread.0)
            .ok_or(QuoteError::OutOfRange)?;
        let ask = mid
            .0
            .checked_add(config.half_spread.0)
            .ok_or(QuoteError::OutOfRange)?;
        Ok(Quote {
            bid: Cents(bid),
            ask: Cents(ask),
            version: config.version,
        })
    }

    /// Called when the store says the config changed. Returns the version quotes now use.
    /// CLUE: `reload` takes `&self` while other threads quote. How do you replace a value behind `&self` while readers may still be using the old one? (Rust Atomics and Locks ch. 1, "Interior Mutability")
    pub fn reload(&self) -> Result<u64, StoreError> {
        Ok(self.store.load()?.version)
    }
}
