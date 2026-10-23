use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use kata_collections_74::{AccountId, Ledger, LedgerError};

// Counts bytes per thread, so tests running in parallel don't disturb each other.
thread_local! {
    static LIVE: Cell<usize> = const { Cell::new(0) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
    static ALLOCS: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = LIVE.try_with(|live| {
            let now = live.get() + layout.size();
            live.set(now);
            let _ = PEAK.try_with(|peak| peak.set(peak.get().max(now)));
            let _ = ALLOCS.try_with(|n| n.set(n.get() + 1));
        });
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = LIVE.try_with(|live| live.set(live.get().saturating_sub(layout.size())));
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOC: Counting = Counting;

/// Runs `f` and returns its result, the peak bytes above the start, and the allocation count.
fn measure<T>(f: impl FnOnce() -> T) -> (T, usize, usize) {
    let start = LIVE.with(Cell::get);
    PEAK.with(|p| p.set(start));
    ALLOCS.with(|a| a.set(0));
    let out = f();
    let peak = PEAK.with(Cell::get).saturating_sub(start);
    let allocs = ALLOCS.with(Cell::get);
    (out, peak, allocs)
}

const BUDGET: usize = 256 * 1024;

fn sparse_id(i: u32) -> AccountId {
    AccountId(i * 10_007)
}

#[test]
fn new_account_starts_at_delta() {
    let mut ledger = Ledger::with_capacity(4);
    assert_eq!(ledger.apply(AccountId(7), 500), Ok(500));
    assert_eq!(ledger.balance(AccountId(7)), Some(500));
}

#[test]
fn unknown_account_has_no_balance() {
    let ledger = Ledger::with_capacity(4);
    assert_eq!(ledger.balance(AccountId(3)), None);
    assert!(ledger.is_empty());
}

#[test]
fn deltas_accumulate_and_go_negative() {
    let mut ledger = Ledger::with_capacity(4);
    assert_eq!(ledger.apply(AccountId(1), 500), Ok(500));
    assert_eq!(ledger.apply(AccountId(1), -700), Ok(-200));
}

#[test]
fn overflow_is_an_error_and_keeps_balance() {
    let mut ledger = Ledger::with_capacity(4);
    let id = AccountId(9);
    assert_eq!(ledger.apply(id, i64::MAX), Ok(i64::MAX));
    assert_eq!(
        ledger.apply(id, 1),
        Err(LedgerError::Overflow { account: id })
    );
    assert_eq!(ledger.balance(id), Some(i64::MAX));
}

#[test]
fn len_counts_distinct_accounts() {
    let mut ledger = Ledger::with_capacity(1000);
    for i in 1..=1000 {
        assert!(ledger.apply(sparse_id(i), 1).is_ok());
        assert!(ledger.apply(sparse_id(i), 1).is_ok());
    }
    assert_eq!(ledger.len(), 1000);
}

#[test]
fn sparse_ids_stay_within_memory_budget() {
    let (ledger, peak, _) = measure(|| {
        let mut ledger = Ledger::with_capacity(1000);
        for i in 1..=1000 {
            assert!(ledger.apply(sparse_id(i), 1).is_ok());
        }
        ledger
    });
    assert_eq!(ledger.len(), 1000);
    assert!(
        peak <= BUDGET,
        "over the memory budget: {peak} bytes for 1000 accounts, budget {BUDGET}"
    );
}

#[test]
fn with_capacity_means_no_growth() {
    let mut ledger = Ledger::with_capacity(1000);
    let ((), _, allocs) = measure(|| {
        for i in 1..=1000 {
            assert!(ledger.apply(sparse_id(i), 1).is_ok());
        }
    });
    assert_eq!(allocs, 0, "grew after with_capacity: {allocs} allocations");
}
