//! Stock levels for one warehouse item, rebuilt by replaying a log of events.

use std::fmt;
use std::hash::{BuildHasher, RandomState};
use std::time::{SystemTime, UNIX_EPOCH};

/// Bytes in one encoded snapshot: four big-endian `u64`s.
pub const SNAPSHOT_LEN: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Received { qty: u32 },
    Shipped { qty: u32 },
}

/// One stored event. `version` is 1, 2, 3, ... per stream. `id` is unique per event.
/// `at_ms` is when the change happened (ms since the Unix epoch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub version: u64,
    pub id: EventId,
    pub at_ms: u64,
    pub change: Change,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Applied,
    Duplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    Gap {
        expected: u64,
        got: u64,
    },
    Oversold {
        version: u64,
        on_hand: u64,
        qty: u32,
    },
    Overflow {
        version: u64,
    },
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplayError::Gap { expected, got } => {
                write!(f, "version gap: expected {expected}, got {got}")
            }
            ReplayError::Oversold {
                version,
                on_hand,
                qty,
            } => {
                write!(f, "version {version}: ship {qty} with {on_hand} on hand")
            }
            ReplayError::Overflow { version } => {
                write!(f, "version {version}: stock overflows u64")
            }
        }
    }
}

impl std::error::Error for ReplayError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    WrongLength { got: usize },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::WrongLength { got } => {
                write!(f, "snapshot is {got} bytes, want {SNAPSHOT_LEN}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stock {
    /// CLUE: The last version applied. What does an event at or below it mean? Two above it?
    version: u64,
    on_hand: u64,
    /// CLUE: You replay last year's log today. What time does this write down?
    updated_at_ms: u64,
    /// CLUE: A new lot number for every receipt. Will tomorrow's replay mint the same one?
    last_lot: u64,
}

impl Stock {
    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn on_hand(&self) -> u64 {
        self.on_hand
    }

    pub fn updated_at_ms(&self) -> u64 {
        self.updated_at_ms
    }

    pub fn last_lot(&self) -> u64 {
        self.last_lot
    }

    pub fn apply(&mut self, event: &Event) -> Result<Outcome, ReplayError> {
        let on_hand = match event.change {
            Change::Received { qty } => {
                self.on_hand
                    .checked_add(u64::from(qty))
                    .ok_or(ReplayError::Overflow {
                        version: event.version,
                    })?
            }
            Change::Shipped { qty } => {
                self.on_hand
                    .checked_sub(u64::from(qty))
                    .ok_or(ReplayError::Oversold {
                        version: event.version,
                        on_hand: self.on_hand,
                        qty,
                    })?
            }
        };
        if let Change::Received { .. } = event.change {
            self.last_lot = RandomState::new().hash_one(event.version);
        }
        self.on_hand = on_hand;
        self.version = event.version;
        self.updated_at_ms = now_ms();
        Ok(Outcome::Applied)
    }

    /// Applies each event in order; stops at the first error.
    pub fn replay_from(&mut self, events: &[Event]) -> Result<(), ReplayError> {
        for event in events {
            self.apply(event)?;
        }
        Ok(())
    }

    /// version, on_hand, updated_at_ms, last_lot as big-endian u64s.
    pub fn encode(&self) -> [u8; SNAPSHOT_LEN] {
        let mut out = [0u8; SNAPSHOT_LEN];
        out[0..8].copy_from_slice(&self.version.to_be_bytes());
        out[8..16].copy_from_slice(&self.on_hand.to_be_bytes());
        out[16..24].copy_from_slice(&self.updated_at_ms.to_be_bytes());
        out[24..32].copy_from_slice(&self.last_lot.to_be_bytes());
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Stock, DecodeError> {
        let b: &[u8; SNAPSHOT_LEN] = bytes
            .try_into()
            .map_err(|_| DecodeError::WrongLength { got: bytes.len() })?;
        Ok(Stock {
            version: word(b, 0),
            on_hand: word(b, 1),
            updated_at_ms: word(b, 2),
            last_lot: word(b, 3),
        })
    }
}

/// `Stock::default()` plus every event.
pub fn replay(events: &[Event]) -> Result<Stock, ReplayError> {
    let mut stock = Stock::default();
    stock.replay_from(events)?;
    Ok(stock)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn word(b: &[u8; SNAPSHOT_LEN], i: usize) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[i * 8..i * 8 + 8]);
    u64::from_be_bytes(w)
}
