use std::collections::HashMap;

/// A place an order can be sent. Prices are whole cents.
/// CLUE: `cheapest` holds venues as `Box<dyn Venue>`. Must every method of the trait be callable through `dyn`?
pub trait Venue {
    fn name(&self) -> &str;

    /// Best ask for `pair` in cents, or `None` if the venue doesn't list it.
    fn best_ask(&self, pair: &str) -> Option<u64>;

    /// Best asks for many pairs at once, in order.
    /// CLUE: A vtable has one slot per method. How many copies of this method does the compiler make, and which one would the slot point to?
    fn best_asks<'a, I>(&self, pairs: I) -> Vec<Option<u64>>
    where
        I: IntoIterator<Item = &'a str>,
    {
        pairs.into_iter().map(|p| self.best_ask(p)).collect()
    }
}

/// A venue with a fixed book of asks.
pub struct FixedVenue {
    name: String,
    asks: HashMap<String, u64>,
}

impl FixedVenue {
    pub fn new(name: &str, asks: &[(&str, u64)]) -> Self {
        FixedVenue {
            name: name.to_string(),
            asks: asks.iter().map(|&(p, a)| (p.to_string(), a)).collect(),
        }
    }
}

impl Venue for FixedVenue {
    fn name(&self) -> &str {
        &self.name
    }

    fn best_ask(&self, pair: &str) -> Option<u64> {
        self.asks.get(pair).copied()
    }
}

/// A venue whose trading is halted: it lists nothing.
pub struct Halted {
    name: String,
}

impl Halted {
    pub fn new(name: &str) -> Self {
        Halted {
            name: name.to_string(),
        }
    }
}

impl Venue for Halted {
    fn name(&self) -> &str {
        &self.name
    }

    fn best_ask(&self, _pair: &str) -> Option<u64> {
        None
    }
}

/// The venue with the lowest ask for `pair`; on a tie, the first one wins.
/// CLUE: Which methods does this function actually call through the trait object?
pub fn cheapest<'v>(venues: &'v [Box<dyn Venue>], pair: &str) -> Option<(&'v str, u64)> {
    venues
        .iter()
        .filter_map(|v| v.best_ask(pair).map(|ask| (v.name(), ask)))
        .min_by_key(|&(_, ask)| ask)
}
