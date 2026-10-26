use std::collections::VecDeque;

/// A market, e.g. one trading pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub u32);

/// A price in whole ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price(pub u64);

/// A quantity in whole lots. On a book update, zero means the level is gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qty(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Bid,
    Ask,
}

/// One event off the matching engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A level's new absolute quantity.
    Book {
        symbol: Symbol,
        side: Side,
        price: Price,
        qty: Qty,
    },
    /// A fill that happened.
    ///
    /// CLUE: A book update says what a level is now. What does a trade say?
    Trade {
        symbol: Symbol,
        price: Price,
        qty: Qty,
    },
}

impl Event {
    pub fn symbol(&self) -> Symbol {
        match self {
            Event::Book { symbol, .. } | Event::Trade { symbol, .. } => *symbol,
        }
    }
}

/// What a client reads from its buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// An event with the feed seq it was pushed with.
    Event { seq: u64, event: Event },
    /// Events up to and including `seq` were lost: fetch a fresh snapshot, then read on.
    ///
    /// CLUE: When nothing more fits, which is worse for the client: a message it never sees, or being told it missed some?
    Resnapshot { seq: u64 },
}

/// One slow client's outbox.
pub struct ClientBuffer {
    capacity: usize,
    queue: VecDeque<Message>,
}

impl ClientBuffer {
    /// A buffer holding at most `capacity` messages (at least 1).
    pub fn new(capacity: usize) -> Self {
        ClientBuffer {
            capacity: capacity.max(1),
            queue: VecDeque::new(),
        }
    }

    /// Queue `event`, pushed by the feed with `seq` (one higher than the last push).
    ///
    /// CLUE: Two updates to one level: does the client need the first? Two updates to different levels of one market?
    pub fn push(&mut self, seq: u64, event: Event) {
        // A slow client only needs the latest state of each market.
        let symbol = event.symbol();
        self.queue
            .retain(|m| !matches!(m, Message::Event { event: e, .. } if e.symbol() == symbol));
        if self.queue.len() >= self.capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(Message::Event { seq, event });
    }

    /// Everything queued, oldest first; the buffer is empty afterwards.
    pub fn drain(&mut self) -> Vec<Message> {
        self.queue.drain(..).collect()
    }
}
