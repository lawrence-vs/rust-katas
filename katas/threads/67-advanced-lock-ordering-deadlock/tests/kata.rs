use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use kata_threads_67::{AccountId, Error, Exchange, OrderId};

#[test]
fn place_reserves_and_cancel_refunds() {
    let ex = Exchange::new();
    ex.deposit(AccountId(1), 100);
    let id = ex.place(AccountId(1), 30).unwrap();
    assert_eq!(ex.balance(AccountId(1)), 70);
    assert_eq!(ex.open_orders(), 1);
    ex.cancel(id).unwrap();
    assert_eq!(ex.balance(AccountId(1)), 100);
    assert_eq!(ex.open_orders(), 0);
}

#[test]
fn errors_are_named() {
    let ex = Exchange::new();
    assert_eq!(ex.place(AccountId(7), 1), Err(Error::UnknownAccount));
    ex.deposit(AccountId(1), 100);
    assert_eq!(ex.place(AccountId(1), 101), Err(Error::InsufficientFunds));
    assert_eq!(ex.balance(AccountId(1)), 100);
    assert_eq!(ex.cancel(OrderId(999)), Err(Error::UnknownOrder));
}

struct Meet {
    armed: AtomicBool,
    calls: AtomicUsize,
    arrived: Mutex<usize>,
    cv: Condvar,
}

#[test]
fn place_and_cancel_in_opposite_order_finish() {
    let meet = Arc::new(Meet {
        armed: AtomicBool::new(false),
        calls: AtomicUsize::new(0),
        arrived: Mutex::new(0),
        cv: Condvar::new(),
    });
    let hook_meet = Arc::clone(&meet);
    let ex = Arc::new(Exchange::with_hook(move || {
        hook_meet.calls.fetch_add(1, Ordering::SeqCst);
        if hook_meet.armed.load(Ordering::SeqCst) {
            let mut n = hook_meet.arrived.lock().unwrap();
            *n += 1;
            hook_meet.cv.notify_all();
            let _ = hook_meet
                .cv
                .wait_timeout_while(n, Duration::from_secs(2), |n| *n < 2)
                .unwrap();
        }
    }));

    ex.deposit(AccountId(1), 100);
    let o1 = ex.place(AccountId(1), 40).unwrap();
    meet.armed.store(true, Ordering::SeqCst);

    let (tx, rx) = mpsc::channel();
    let (ex_a, tx_a) = (Arc::clone(&ex), tx.clone());
    thread::spawn(move || {
        let _ = tx_a.send(ex_a.place(AccountId(1), 10).map(|_| ()));
    });
    let (ex_b, tx_b) = (Arc::clone(&ex), tx);
    thread::spawn(move || {
        let _ = tx_b.send(ex_b.cancel(o1));
    });

    // No join or scope: a deadlocked thread must fail the test, not hang it.
    for _ in 0..2 {
        let result = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("place and cancel deadlocked");
        assert_eq!(result, Ok(()));
    }
    assert_eq!(ex.balance(AccountId(1)), 90);
    assert_eq!(ex.open_orders(), 1);
    assert_eq!(
        meet.calls.load(Ordering::SeqCst),
        3,
        "place and cancel each call the hook once"
    );
}

#[test]
fn many_threads_place_and_cancel_without_hanging() {
    let ex = Arc::new(Exchange::new());
    for a in 0..4 {
        ex.deposit(AccountId(a), 1_000);
    }
    let (tx, rx) = mpsc::channel();
    for i in 0..8u32 {
        let ex = Arc::clone(&ex);
        let tx = tx.clone();
        thread::spawn(move || {
            for _ in 0..1_000 {
                let id = ex.place(AccountId(i % 4), 1).unwrap();
                ex.cancel(id).unwrap();
            }
            let _ = tx.send(());
        });
    }
    for _ in 0..8 {
        rx.recv_timeout(Duration::from_secs(10))
            .expect("place and cancel deadlocked under load");
    }
    for a in 0..4 {
        assert_eq!(ex.balance(AccountId(a)), 1_000);
    }
    assert_eq!(ex.open_orders(), 0);
}
