# 62 advanced: out-of-order and duplicate events

**Concept:** Arrival order isn't event order: a per-order version ignores stale and duplicate messages, buffers early ones (bounded), and a bad transition is an error, not a panic.
**Source:** proptest book (`prop_shuffle`); thiserror docs; kata 50, kata 52; own design (an exchange's order-update consumer).
**Fails with:** `test a_fill_before_its_accept_is_buffered ... FAILED`
**Vault:** your note on event ordering.

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect` or `panic!` in `src/`. Don't change `State::apply` or any public signature. Bound memory by `max_pending`.

## Before you fix it
1. Which tests already pass, and why?
2. The transition table is correct. So what is the bug, and where does it live?

## Answer in the PR
1. Why buffer an early message instead of rejecting it?
2. Why does a full buffer refuse the new message instead of evicting an old one?
3. Why is ignoring `version <= applied` safe here but not with cumulative quantities?
4. What does `unwrap` cost in a consumer?
5. Why one version per order and not one global sequence?

## Think further
1. Two different messages with the same version.
2. A gap that never fills: a timeout, then a resync from the order-status endpoint (kata 52).
3. Buffering by distance as well as by count.
4. Total memory across many orders (kata 21).
5. `CumQty`-style fills, where a replayed fill is not harmless.
6. Restart and replay (kata 50).
7. A version at `u64::MAX`.

## In the wild
- FIX `ExecutionReport` (`OrdStatus`, `CumQty`).
- Exchange private WebSocket order-update streams.
- Kafka, which orders messages per partition key only.
- At-least-once queues that redeliver out of order.
- Optimistic concurrency versions in event sourcing.

In a review, check:
- `unwrap` on a transition.
- No version check.
- An unbounded reorder buffer.
- Deduplication only on success.
- A late event overwriting a final state.
