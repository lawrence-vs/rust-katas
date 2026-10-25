//! Closes a wallet ledger in parallel.
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryId(pub u32);

/// Signed whole cents: a deposit is positive, a withdrawal negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cents(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub id: EntryId,
    pub amount: Cents,
}

/// One statement line: `"<id> <amount in cents>"`.
pub fn line(entry: &Entry) -> String {
    format!("{} {}", entry.id.0, entry.amount.0)
}

/// `opening` plus every amount, or `None` if the exact total doesn't fit in i64.
/// CLUE: rayon calls the `try_fold` identity once per piece, and even a 1-thread pool cuts the slice in two. How many times is `opening` added?
/// CLUE: `[i64::MAX, 1, -1]` overflows if you add left to right, but not if you split it as `[MAX] | [1, -1]`. Which type makes the grouping irrelevant?
pub fn closing_balance(opening: Cents, entries: &[Entry]) -> Option<Cents> {
    entries
        .par_iter()
        .try_fold(|| opening.0, |acc, e| acc.checked_add(e.amount.0))
        .try_reduce(|| 0, |a, b| a.checked_add(b))
        .map(Cents)
}

/// Every entry's `line`, in input order, joined by `"\n"`.
/// CLUE: Is `String::new()` an identity for `|a, b| a + "\n" + &b`? Work out `op("", "0 500")`.
pub fn statement(entries: &[Entry]) -> String {
    entries
        .par_iter()
        .map(line)
        .reduce(String::new, |a, b| a + "\n" + &b)
}
