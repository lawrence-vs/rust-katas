# 51 credit (intermediate): a credit line overspent

**Concept:** Checking the credit and reserving it must be one atomic step, and a fill or cancel gives back the credit it doesn't use. A check followed by a separate write lets two orders both see the same headroom.
**Source:** Rust Atomics and Locks (Mara Bos), ch. 2 "Compare-and-Exchange Operations", https://marabos.nl/atomics/atomics.html; [loom](https://github.com/tokio-rs/loom); katas 41 and 43
**Fails with:** `two orders overspent the credit line`
**Vault:** your note on check-then-act

The library uses loom's atomics so the tests can explore every interleaving, so it only works inside `loom::model`. A plain thread loop would often pass on x86 and fail now and then; loom fails the same way on every run and every CPU.

## Task
Make `cargo test` pass without changing the tests. Keep the public API. No `unwrap`, `expect`, `panic!` or `wrapping_*` in `src/`. No new crates; use one lock or one compare-and-exchange.

## Before you fix it
1. Run `cargo test`. Which tests pass, and why?
2. In the race test, write the interleaving that lets both orders through.

## Answer in the PR
1. The failing interleaving: which loads and stores ran, and how much was committed against a 100 limit?
2. Lock or compare-and-exchange: which did you pick, and what does the other cost?
3. Why is `Relaxed` enough for one counter changed by read-modify-write, or why did you pick stronger? (kata 43)
4. Why do `fill` and `cancel` take `Reservation` by value, and why doesn't it derive `Clone`?
5. Why is a `fetch_add` followed by a `fetch_sub` undo when over the limit wrong for a credit line, even though the counter ends right?

## Think further
1. A dropped `Reservation` leaks credit. How would RAII fix it (kata 39), and what would `Drop` have to do?
2. What happens if a reservation from one line is passed to another line's `cancel`?
3. Repayment of filled credit should lower `committed`. What changes?
4. The risk desk cuts the limit while orders are open. What should `available` say?
5. Many clients: a lock per client, or one lock around a map? What does each cost?
6. The same check in a database: `UPDATE … SET committed = committed + $1 WHERE committed + $1 <= limit` against SELECT then UPDATE.

## In the wild
- Pre-trade credit and margin checks at exchanges and brokers: an order is refused before it reaches the book if it would take the account past its limit.
- Prime-broker credit limits shared by many trading desks.
- `tokio::sync::Semaphore::try_acquire_many`: permits are a credit line.
- Conditional database updates and `SELECT … FOR UPDATE`.

In a review, check:
- A load then a store on one atomic.
- SELECT then UPDATE with no lock or condition.
- A release that can be called twice.
- A release with an unchecked `-`.
