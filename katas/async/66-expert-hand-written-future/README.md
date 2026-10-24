# 66 expert: a hand-written future that never wakes

**Concept:** a future that returns `Pending` must leave its waker where the event can find it. If nobody holds the waker, nobody polls the task again and it hangs.
**Source:** Rust for Rustaceans, ch 8 "Asynchronous Programming" (waking); [`std::task::Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html); [`Future::poll`](https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll)
**Fails with:** `the waiter was never woken`
**Vault:** your note on wakers

## Task
Make `cargo test` pass without changing the tests. Std only in `src/`; no `unwrap`, `expect` or `panic!` in `src/`; don't busy-loop with `wake_by_ref()` from `poll`. Most tests run with paused time, so a hang fails at once.

## Before you fix it
1. Which test passes already, and why?
2. What does `confirm` find in `waker`?

## Answer in the PR
1. Walk the starter through `a_parked_waiter_is_woken`: who polls, who confirms, and why does the runtime then sit idle?
2. Why store the waker on every `Pending` and not only the first?
3. Calling `cx.waker().wake_by_ref()` before returning `Pending` also makes the tests pass. What does it cost?
4. Why does `confirmed_before_await_is_ready` pass on broken code, and what does that say about testing futures?

## Think further
1. Is it safe to call `wake()` while holding the lock? What could the waker do?
2. `Waker::will_wake` and `clone_from` can skip a clone. When is that worth it?
3. A dropped `Confirmer` leaves the waiter hanging forever. Change `Output` to `Result<u64, Canceled>` and wake in `Drop`.
4. Why is `Confirmed` `Unpin`, and what would change if it were not?
5. What does `AtomicWaker` do that a mutex around an `Option<Waker>` does not?

## In the wild
- [tokio `sync::oneshot`](https://docs.rs/tokio/1.53.2/tokio/sync/oneshot/index.html): the production version of this kata.
- [futures `AtomicWaker`](https://docs.rs/futures/0.3.31/futures/task/struct.AtomicWaker.html): a lock-free slot for the latest waker.
- [`std::task::Wake`](https://doc.rust-lang.org/std/task/trait.Wake.html): build a `Waker` from an `Arc` without unsafe.
- Exchange-style: an order gateway awaiting an ack from the matching engine. A lost wake is an order that never times out and never fills.

In a review, check:
- A `Pending` return with no waker stored.
- The waker stored only once, so a future moved to another task is never woken.
- `wake()` called under a lock.
- A manual `poll` impl where an `async fn` would do.
