# 67 advanced: two locks taken in opposite orders

**Concept:** Two threads that each take the same two locks in opposite orders can each hold one and wait forever for the other. The fix is to make that impossible: one order for every lock, one lock around the state that changes together, or one owner thread that gets messages.
**Source:** Rust for Rustaceans (Gjengset), ch. 10 Concurrency (and Parallelism), the correctness problems of shared memory and the actor/worker alternatives (no Readwise highlight was available to the builder); std [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) docs
**Fails with:** `place and cancel deadlocked`
**Vault:** your note on deadlock and lock order

A deadlock compiles, so the failure is a test that times out. A loop of threads only sometimes deadlocks, and a kata must fail every run. So `Exchange::with_hook` takes a hook that `place` and `cancel` call while holding their first lock. The test uses the hook as a meeting point: both threads hold one lock before either asks for the second.

## Task
Make `cargo test` pass without changing the tests. Keep the public API, and call the hook once per `place` and `cancel`. Each of `place` and `cancel` still changes balances and orders as one step: don't release one lock before taking the other. No `try_lock` retry loops, no `unsafe`, no new crates, no `unwrap`/`expect` in `src/`.

## Before you fix it
1. Run `cargo test`. Which test fails, and after how long?
2. Draw the two threads: what does each hold, and what is each waiting for?

## Answer in the PR
1. Name the interleaving: which guard each thread held and which lock it waited on.
2. Which of the three fixes did you pick, and what does each of the other two cost? (Contention on one lock; a channel hop and an owner thread; an order the compiler doesn't check.)
3. Why not drop the `balances` guard before locking `orders`? What could another thread observe?
4. Why does the test use `spawn` plus `recv_timeout` and not `thread::scope` or `join`?
5. Why does the test need the meeting point to fail every run, when a loop of 8 threads often deadlocks anyway?

## Think further
1. A transfer between two accounts, each with its own `Mutex`: `transfer(a, b)` and `transfer(b, a)`. What order do you pick when both locks are the same kind?
2. The hook runs user code while a lock is held. What happens if a hook calls `balance()`? What does std promise about locking a `Mutex` twice on one thread?
3. Does `tokio::sync::Mutex` change any of this?
4. An `RwLock` reader that wants to become a writer: why can that deadlock with only one lock?

## In the wild
- Linux lockdep, [Runtime locking correctness validator](https://docs.kernel.org/locking/lockdep-design.html): records lock order and reports inversions before they hang.
- [`parking_lot`](https://docs.rs/parking_lot/latest/parking_lot/deadlock/index.html)'s `deadlock_detection` feature.
- PostgreSQL's `deadlock detected` error, which aborts one transaction ([explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)).
- Exchange-style: a single-threaded matching engine per instrument (single writer, actor style).

In a review, check:
- Two `.lock()` calls in one function.
- The same pair of locks taken in a different order elsewhere.
- A callback or I/O called while a guard is held.
