//! An order gateway that caps how many orders are in flight at once.

use std::cell::Cell;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum SubmitError {
    /// Every in-flight slot is taken.
    Busy,
    /// The quantity wasn't a whole number.
    BadQuantity(ParseIntError),
    /// A quantity of zero.
    ZeroQuantity,
}

impl From<ParseIntError> for SubmitError {
    fn from(e: ParseIntError) -> Self {
        SubmitError::BadQuantity(e)
    }
}

pub struct Gateway {
    limit: usize,
    in_flight: Cell<usize>,
}

impl Gateway {
    pub fn new(limit: usize) -> Self {
        Gateway {
            limit,
            in_flight: Cell::new(0),
        }
    }

    /// Orders currently holding a slot.
    pub fn in_flight(&self) -> usize {
        self.in_flight.get()
    }

    /// Takes one in-flight slot, or `Busy` if all `limit` are taken.
    /// CLUE: What if this returned a value whose job is to give the slot back? Which trait runs code when a value goes out of scope?
    /// CLUE: That value has to reach `in_flight` later. What would it borrow, and what does that put on its type?
    pub fn reserve(&self) -> Result<(), SubmitError> {
        if self.in_flight.get() >= self.limit {
            return Err(SubmitError::Busy);
        }
        self.in_flight.set(self.in_flight.get() + 1);
        Ok(())
    }

    /// Gives a slot back. Every `reserve` must be paired with exactly one `release`.
    /// CLUE: Every caller has to remember this on every way out, a panic included. Who could remember it for them?
    pub fn release(&self) {
        self.in_flight.set(self.in_flight.get() - 1);
    }

    /// Checks a raw quantity such as `"25"` and sends the order, holding a slot while it does.
    /// CLUE: `?` and `return` both leave this function. Which line never runs on those paths?
    pub fn submit(&self, raw_qty: &str) -> Result<u64, SubmitError> {
        self.reserve()?;
        let qty: u64 = raw_qty.trim().parse()?;
        if qty == 0 {
            return Err(SubmitError::ZeroQuantity);
        }
        // Sending to the venue would happen here.
        self.release();
        Ok(qty)
    }
}
