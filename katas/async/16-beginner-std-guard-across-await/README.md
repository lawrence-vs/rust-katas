# 16 beginner: a std MutexGuard across `.await`

**Concept:** A `std::sync::MutexGuard` held across `.await` makes the future non-`Send`, so `tokio::spawn` rejects it, and if it did run it could block a worker. Keep std locks short: lock, change, drop, then await.
**Source:** Tokio tutorial, shared state (holding a MutexGuard across an .await), https://tokio.rs/tokio/tutorial/shared-state
**Fails with:** `future cannot be sent between threads safely`
**Vault:** your note on locks and await points

## Task
Make `cargo test` pass without changing the test. Keep `std::sync::Mutex`: the critical section is short and synchronous.

## Before you fix it
1. Read the error. Which value does it say is held across which `.await`?
2. Why does `tokio::spawn` need `Send`?

## Answer in the PR
1. Why is a guard held across `.await` dangerous even on one thread?
2. Switching to `tokio::sync::Mutex` also compiles. Why is std still the right lock here?
3. When is `tokio::sync::Mutex` the right lock?

## Think further
1. `audit` only needs the new count. What changes if it needed something that isn't `Copy`, like a `String` built from data inside the lock?
2. `lock().unwrap()` panics if the mutex is poisoned. What poisons it, which task would see the panic, and what should `bump_many` do then?
3. Ten thousand tasks bump one counter. Is `Mutex<u64>` still the right tool? What would `AtomicU64` change, and what would it not give you? (kata 43 goes into orderings)

## In the wild
- [tokio `Mutex`, "Which kind of mutex should you use?"](https://docs.rs/tokio/1.53.2/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use): tokio's own advice on std versus async locks.
- [std `MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html): check "Auto Trait Implementations" for `!Send`. That is where this kata's compile error comes from.
- [Clippy `await_holding_lock`](https://clippy.rust-lang.org/master/index.html#await_holding_lock): a lint that finds this bug before `tokio::spawn` does. Read which guard types it covers. (`await_holding_refcell_ref` is its sibling.)
- [mini-redis `src/db.rs`](https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs): the tokio team's example server keeps its shared state behind a std `Mutex`. Read the comment above it.
- Exchange-style: a per-account sequence number or balance is locked for a few instructions on every order. The journal or audit write that follows is I/O.

In a review, check:
- A `std::sync::MutexGuard`, `parking_lot` guard or `RefCell` borrow still alive at an `.await`. Is `clippy::await_holding_lock` on in CI?
- A switch from `std::sync::Mutex` to `tokio::sync::Mutex` with no reason given.
- `lock().unwrap()` in library code with no word about poisoning.
