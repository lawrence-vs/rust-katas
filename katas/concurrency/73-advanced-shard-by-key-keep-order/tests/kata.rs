use kata_concurrency_73::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn no_hook(_: &Event) {}

fn ev(account: u64, seq: u64, op: Op) -> Event {
    Event {
        account: AccountId(account),
        seq,
        op,
    }
}

#[test]
fn interleaved_accounts_keep_per_account_order() {
    let events: Vec<Event> = (0..30u64)
        .map(|i| {
            let k = i / 3;
            let op = if k % 2 == 0 {
                Op::Deposit(10)
            } else {
                Op::Withdraw(10)
            };
            ev(1 + i % 3, i, op)
        })
        .collect();
    let ledger = process_sharded(events.clone(), 4, 2, no_hook).unwrap();
    for account in 1..=3u64 {
        let expected: Vec<u64> = (0..30u64).filter(|i| 1 + i % 3 == account).collect();
        let got = ledger.get(&AccountId(account)).unwrap();
        assert_eq!(
            got.applied, expected,
            "interleaved accounts keep per-account order"
        );
        assert!(got.rejected.is_empty());
    }
    assert_eq!(ledger, process_sequential(events));
}

fn lcg_events(n: u64) -> Vec<Event> {
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    (0..n)
        .map(|seq| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let account = (state >> 33) % 37;
            let amount = (state >> 20) % 50;
            let op = if (state >> 60) & 1 == 0 {
                Op::Deposit(amount)
            } else {
                Op::Withdraw(amount)
            };
            ev(account, seq, op)
        })
        .collect()
}

#[test]
fn matches_a_single_threaded_run() {
    let events = lcg_events(5_000);
    let expected = process_sequential(events.clone());
    for workers in [1, 2, 3, 8, 64] {
        for capacity in [0, 1, 16] {
            let got = process_sharded(events.clone(), workers, capacity, no_hook).unwrap();
            assert!(
                got == expected,
                "matches a single-threaded run (workers {workers}, capacity {capacity})"
            );
        }
    }
}

#[test]
fn the_producer_waits_for_a_slow_worker() {
    let pulled = AtomicUsize::new(0);
    let events = (0..10_000u64).map(|seq| {
        pulled.fetch_add(1, Ordering::SeqCst);
        ev(5, seq, Op::Deposit(1))
    });
    let (entered_tx, entered_rx) = mpsc::channel::<()>();
    let (release_tx, release_rx) = mpsc::channel::<()>();
    let release_rx = Mutex::new(release_rx);
    let entered_tx = Mutex::new(entered_tx);
    let hook = |e: &Event| {
        if e.seq == 0 {
            entered_tx.lock().unwrap().send(()).unwrap();
            let _ = release_rx.lock().unwrap().recv();
        }
    };
    let peak = AtomicUsize::new(0);
    let ledger = thread::scope(|s| {
        let (pulled, peak) = (&pulled, &peak);
        s.spawn(move || {
            let _ = entered_rx.recv_timeout(Duration::from_secs(5));
            thread::sleep(Duration::from_millis(200));
            peak.store(pulled.load(Ordering::SeqCst), Ordering::SeqCst);
            let _ = release_tx.send(());
        });
        process_sharded(events, 4, 8, hook)
    })
    .unwrap();
    let peak = peak.load(Ordering::SeqCst);
    assert!(
        peak <= 8 + 2,
        "the producer waits for a slow worker (pulled {peak} events while one worker stalled)"
    );
    assert_eq!(ledger.get(&AccountId(5)).unwrap().balance, 10_000);
}

#[test]
fn accounts_run_in_parallel() {
    let events = vec![ev(1, 0, Op::Deposit(1)), ev(2, 1, Op::Deposit(1))];
    let (tx, rx) = mpsc::channel::<()>();
    let tx = Mutex::new(tx);
    let rx = Mutex::new(rx);
    let arrived = AtomicBool::new(false);
    let hook = |e: &Event| {
        if e.account == AccountId(2) {
            let _ = tx.lock().unwrap().send(());
        } else {
            let got = rx.lock().unwrap().recv_timeout(Duration::from_secs(2));
            arrived.store(got.is_ok(), Ordering::SeqCst);
        }
    };
    process_sharded(events, 2, 1, hook).unwrap();
    assert!(arrived.load(Ordering::SeqCst), "accounts run in parallel");
}

#[test]
fn a_panicking_worker_is_an_error() {
    let events: Vec<Event> = (0..1_000u64)
        .map(|i| ev(1 + i % 3, i, Op::Deposit(1)))
        .collect();
    let hook = |e: &Event| {
        if e.account == AccountId(2) && e.seq >= 500 {
            panic!("worker dies");
        }
    };
    assert_eq!(
        process_sharded(events, 3, 1, hook),
        Err(ShardError::WorkerPanicked)
    );
}

#[test]
fn zero_workers_is_an_error() {
    assert_eq!(
        process_sharded(Vec::new(), 0, 1, no_hook),
        Err(ShardError::NoWorkers)
    );
}
