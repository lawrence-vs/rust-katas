//! Marks a book of positions to market in parallel.
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PositionId(pub u32);

/// Quote-currency cents per unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cents(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub id: PositionId,
    pub qty: u64,
}

/// `notional` is qty × mark, in cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Valuation {
    pub id: PositionId,
    pub notional: u128,
}

/// One valuation per position, in the book's order. `mark` is called once per position.
/// CLUE: Which thread pushes first? What does `for_each` promise about order, and what does an indexed `collect` promise?
/// CLUE: Every worker takes the same lock for every item. How much of this loop actually runs in parallel?
pub fn value_book<F>(book: &[Position], mark: F) -> Vec<Valuation>
where
    F: Fn(&Position) -> Cents + Sync,
{
    let out = Arc::new(Mutex::new(Vec::with_capacity(book.len())));
    book.par_iter().for_each(|p| {
        let notional = u128::from(p.qty) * u128::from(mark(p).0);
        out.lock().unwrap().push(Valuation { id: p.id, notional });
    });
    Arc::try_unwrap(out).unwrap().into_inner().unwrap()
}

/// Sum of every notional, or `None` if it overflows u128.
/// CLUE: rayon docs, `reduce`/`try_reduce`: what must the identity and the operation satisfy so the split doesn't change the answer?
pub fn total_exposure<F>(book: &[Position], mark: F) -> Option<u128>
where
    F: Fn(&Position) -> Cents + Sync,
{
    let total = Mutex::new(Some(0u128));
    book.par_iter().for_each(|p| {
        let n = u128::from(p.qty) * u128::from(mark(p).0);
        let mut t = total.lock().unwrap();
        *t = t.and_then(|t| t.checked_add(n));
    });
    total.into_inner().unwrap()
}
