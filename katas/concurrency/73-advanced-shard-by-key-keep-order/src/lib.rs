//! Account events, applied on worker threads.

use std::collections::BTreeMap;
use std::fmt;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Deposit(u64),
    Withdraw(u64),
}

/// One event off the stream. `seq` is its position in the whole stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub account: AccountId,
    pub seq: u64,
    pub op: Op,
}

/// `applied` and `rejected` hold seqs in the order this account saw them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Account {
    pub balance: u64,
    pub applied: Vec<u64>,
    pub rejected: Vec<u64>,
}

pub type Ledger = BTreeMap<AccountId, Account>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShardError {
    NoWorkers,
    WorkerPanicked,
}

impl fmt::Display for ShardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShardError::NoWorkers => write!(f, "at least one worker is needed"),
            ShardError::WorkerPanicked => write!(f, "a worker panicked"),
        }
    }
}

impl std::error::Error for ShardError {}

/// Deposit: checked_add, None => rejected. Withdraw > balance => rejected. Otherwise applied.
pub fn apply(ledger: &mut Ledger, event: &Event) {
    let account = ledger.entry(event.account).or_default();
    let new_balance = match event.op {
        Op::Deposit(n) => account.balance.checked_add(n),
        Op::Withdraw(n) => account.balance.checked_sub(n),
    };
    match new_balance {
        Some(balance) => {
            account.balance = balance;
            account.applied.push(event.seq);
        }
        None => account.rejected.push(event.seq),
    }
}

/// The reference: one thread, stream order.
pub fn process_sequential(events: impl IntoIterator<Item = Event>) -> Ledger {
    let mut ledger = Ledger::new();
    for event in events {
        apply(&mut ledger, &event);
    }
    ledger
}

/// Applies `events` on `workers` threads. Must equal `process_sequential` for any input,
/// keep each account's events in stream order, run different accounts in parallel, and hold
/// at most `capacity` queued events per worker (0 = rendezvous). Calls `on_applied` on the
/// worker thread after each event. A panicking worker gives `Err(WorkerPanicked)`, never a
/// hang or a panic out of this function.
/// CLUE: Events 0 and 3 are both for account 1. Which worker gets each one, and which ledger sees the deposit before the withdraw?
/// CLUE: How many events are in memory before the first worker starts? What would make the producer wait for a slow worker?
/// CLUE: If a worker dies while you're still sending to it, what does `send` return, and which threads still need joining?
pub fn process_sharded<I, F>(
    events: I,
    workers: usize,
    _capacity: usize,
    on_applied: F,
) -> Result<Ledger, ShardError>
where
    I: IntoIterator<Item = Event>,
    F: Fn(&Event) + Sync,
{
    if workers == 0 {
        return Err(ShardError::NoWorkers);
    }
    let mut batches: Vec<Vec<Event>> = vec![Vec::new(); workers];
    for (i, event) in events.into_iter().enumerate() {
        batches[i % workers].push(event);
    }
    let on_applied = &on_applied;
    thread::scope(|s| {
        let handles: Vec<_> = batches
            .into_iter()
            .map(|batch| {
                s.spawn(move || {
                    let mut ledger = Ledger::new();
                    for event in &batch {
                        apply(&mut ledger, event);
                        on_applied(event);
                    }
                    ledger
                })
            })
            .collect();
        let results: Vec<_> = handles.into_iter().map(|h| h.join()).collect();
        let mut merged = Ledger::new();
        // CLUE: Two workers each built an `Account` for the same id. Can any merge give back the order things happened in?
        for result in results {
            let Ok(part) = result else {
                return Err(ShardError::WorkerPanicked);
            };
            for (id, acc) in part {
                let into = merged.entry(id).or_default();
                into.balance = into.balance.saturating_add(acc.balance);
                into.applied.extend(acc.applied);
                into.rejected.extend(acc.rejected);
            }
        }
        Ok(merged)
    })
}
