use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Accepted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

/// One message off the venue's order-update stream. `version` is per order: 1, 2, 3, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    pub order: OrderId,
    pub version: u64,
    pub event: Event,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TransitionError {
    #[error("{event:?} is not valid in {from:?}")]
    Invalid { from: State, event: Event },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConsumerError {
    #[error("order {order:?} version {version}: {source}")]
    Transition {
        order: OrderId,
        version: u64,
        source: TransitionError,
    },
    #[error("order {order:?}: waiting for version {expected}, buffer full at {buffered}")]
    GapTooLong {
        order: OrderId,
        expected: u64,
        buffered: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Applied, plus any buffered messages that now follow in order; `state` is the state after the last one.
    Applied { state: State },
    /// Arrived early; held until the versions before it arrive (or already held).
    Buffered,
    /// At or below the applied version: a duplicate or stale message. Nothing changed.
    Ignored,
}

impl State {
    pub fn apply(&self, event: Event) -> Result<State, TransitionError> {
        match (self, event) {
            (State::New, Event::Accepted) => Ok(State::Open),
            (State::New, Event::Rejected) => Ok(State::Rejected),
            (State::Open | State::PartiallyFilled, Event::PartiallyFilled) => {
                Ok(State::PartiallyFilled)
            }
            (State::Open | State::PartiallyFilled, Event::Filled) => Ok(State::Filled),
            (State::Open | State::PartiallyFilled, Event::Cancelled) => Ok(State::Cancelled),
            (from, event) => Err(TransitionError::Invalid { from: *from, event }),
        }
    }
}

struct Tracker {
    state: State,
    /// CLUE: The highest version applied. What does a message at or below it tell you? Two above it?
    version: u64,
}

impl Tracker {
    fn new() -> Self {
        Tracker {
            state: State::New,
            version: 0,
        }
    }
}

pub struct Orders {
    #[allow(dead_code)]
    max_pending: usize,
    orders: HashMap<OrderId, Tracker>,
}

impl Orders {
    pub fn new(max_pending: usize) -> Self {
        Orders {
            max_pending,
            orders: HashMap::new(),
        }
    }

    /// CLUE: Arrival order isn't event order. Where can a message wait for the one before it, and how many may wait?
    pub fn on_message(&mut self, msg: Message) -> Result<Outcome, ConsumerError> {
        let tracker = self.orders.entry(msg.order).or_insert_with(Tracker::new);
        // CLUE: `unwrap` turns a bad message from the venue into your crash. Who should decide what happens next?
        let next = tracker.state.apply(msg.event).unwrap();
        tracker.state = next;
        tracker.version = msg.version;
        Ok(Outcome::Applied { state: next })
    }

    /// `None` until the first message for the order.
    pub fn state(&self, order: OrderId) -> Option<State> {
        self.orders.get(&order).map(|t| t.state)
    }

    /// The last applied version.
    pub fn version(&self, order: OrderId) -> Option<u64> {
        self.orders.get(&order).map(|t| t.version)
    }

    /// How many messages are held for the order; 0 for unknown orders.
    pub fn buffered(&self, _order: OrderId) -> usize {
        0
    }
}
