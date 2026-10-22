use std::collections::HashSet;

/// The client's idempotency key: one per transfer, sent again on every retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransferKey(pub u64);

/// An amount in minor units (cents).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cents(pub u64);

/// The rail gave no answer in time. The money may or may not have moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

/// The payment rail that moves the money. Not idempotent: every call is a transfer.
pub trait Rail {
    fn send(&mut self, key: TransferKey, amount: Cents) -> Result<(), TimedOut>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// The client asks for a transfer, or retries one.
    Requested { key: TransferKey, amount: Cents },
    /// The rail reports the money arrived.
    Settled { key: TransferKey },
    /// The rail reports the transfer was refused.
    Rejected { key: TransferKey },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Sent and accepted by the rail; the outcome is pending.
    InFlight,
    /// Sent, but the call timed out: nobody knows if the money moved.
    Unknown,
    Settled,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferError {
    /// An outcome for a key that was never requested.
    UnknownTransfer(TransferKey),
    /// The key was already used for a different amount.
    KeyReused(TransferKey),
    /// The event contradicts a final state; the transfer keeps `state`.
    Conflict { key: TransferKey, state: State },
}

/// CLUE: This only remembers settled transfers. What does it know about one whose send timed out?
pub struct Transfers<R> {
    rail: R,
    settled: HashSet<TransferKey>,
}

impl<R: Rail> Transfers<R> {
    pub fn new(rail: R) -> Self {
        Self {
            rail,
            settled: HashSet::new(),
        }
    }

    pub fn rail(&self) -> &R {
        &self.rail
    }

    /// The transfer's current state, or `None` if the key was never requested.
    pub fn state(&self, key: TransferKey) -> Option<State> {
        self.settled.contains(&key).then_some(State::Settled)
    }

    /// Apply one event and return the transfer's state after it.
    /// Applying an event a second time changes nothing and sends nothing.
    /// CLUE: A timeout means "no answer", not "not sent". What should a retry with the same key do?
    /// CLUE: Which states can a rail outcome still move a transfer out of, and which are final?
    pub fn apply(&mut self, event: Event) -> Result<State, TransferError> {
        match event {
            Event::Requested { key, amount } => {
                if self.settled.contains(&key) {
                    return Ok(State::Settled);
                }
                match self.rail.send(key, amount) {
                    Ok(()) => Ok(State::InFlight),
                    Err(TimedOut) => Ok(State::Unknown),
                }
            }
            Event::Settled { key } => {
                self.settled.insert(key);
                Ok(State::Settled)
            }
            Event::Rejected { .. } => Ok(State::Rejected),
        }
    }
}
