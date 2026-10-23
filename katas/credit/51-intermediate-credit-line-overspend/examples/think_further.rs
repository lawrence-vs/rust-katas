//! 51 credit (intermediate): a credit line overspent
//!
//! Think further, from the kata README:
//! 1. A dropped `Reservation` leaks credit. How would RAII fix it (kata 39), and what would `Drop` have to do?
//! 2. What happens if a reservation from one line is passed to another line's `cancel`?
//! 3. Repayment of filled credit should lower `committed`. What changes?
//! 4. The risk desk cuts the limit while orders are open. What should `available` say?
//! 5. Many clients: a lock per client, or one lock around a map? What does each cost?
//! 6. The same check in a database: `UPDATE … SET committed = committed + $1 WHERE committed + $1 <= limit` against SELECT then UPDATE.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug, PartialEq, Eq)]
enum CreditError {
    OverLimit,
    UnknownReservation,
}

#[derive(Debug)]
struct Line {
    id: u32,
    limit: u64,
    committed: u64,
}

// Think further 1: a reservation that the caller must hand back, so it can't be dropped silently.
#[derive(Debug)]
struct Reservation {
    line_id: u32,
    amount: u64,
}

impl Line {
    fn available(&self) -> u64 {
        self.limit.saturating_sub(self.committed)
    }

    fn reserve(&mut self, amount: u64) -> Result<Reservation, CreditError> {
        if amount > self.available() {
            return Err(CreditError::OverLimit);
        }
        self.committed += amount;
        Ok(Reservation { line_id: self.id, amount })
    }

    // Think further 2: a reservation from another line is refused, not released here.
    fn cancel(&mut self, r: Reservation) -> Result<(), CreditError> {
        if r.line_id != self.id {
            return Err(CreditError::UnknownReservation);
        }
        self.committed -= r.amount;
        Ok(())
    }

    // Think further 3: repayment lowers committed, so later orders can use the room again.
    fn repay(&mut self, amount: u64) {
        self.committed = self.committed.saturating_sub(amount);
    }
}

fn main() {
    let mut line = Line { id: 1, limit: 1_000, committed: 0 };
    let mut other = Line { id: 2, limit: 500, committed: 0 };

    let r = line.reserve(600).expect("fits the limit");
    println!("reserve 600 again: {:?}", line.reserve(600));
    println!("cancel on the wrong line: {:?}", other.cancel(r));

    let r = line.reserve(300).expect("fits the limit");
    println!("cancel on the right line: {:?}", line.cancel(r));

    line.repay(200);
    println!("after repay, available {}", line.available());
}
