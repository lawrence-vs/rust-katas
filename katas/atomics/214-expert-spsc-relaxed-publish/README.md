# 214 expert: an SPSC ring buffer that publishes with `Relaxed`

**Concept:** the consumer reads a slot after it sees `tail` move. Only a `Release` store on `tail` paired with an `Acquire` load makes the slot write visible. `Relaxed` on the publish gives a data race.
**Source:** The Rustonomicon, atomics and ordering, https://doc.rust-lang.org/nomicon/atomics.html; `std::sync::atomic::Ordering` docs, https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
**Fails with:** `Undefined Behavior: Data race detected` under Miri. Plain `cargo test` may pass, so run Miri. CI checks for `Data race detected`.
**Vault:** none yet; write one on which `Ordering` each store needs and why.

## Task
Make `cargo +nightly miri test` pass without changing the tests or the public API of `Spsc`. Change only what the atomics need. No `Mutex`, no `VecDeque`.

## Before you fix it
1. Read the Miri error. Which two threads and which two accesses race?
2. Run `cargo test` once. Does it pass? What does that tell you about the bug?

## Answer in the PR
1. Which store publishes the slot to the consumer, and which load must pair with it?
2. Why does the `Relaxed` store on `tail` compile and run without error on most machines?
3. Why is `Relaxed` fine for the producer's own `tail` load?

## Think further
1. The consumer's `head` store is `Release`. Does it need to be? What would go wrong if it were `Relaxed`?
2. Usable capacity is `cap - 1`. Why does this design need one empty slot?

## Related concepts
- `Mutex<VecDeque<T>>`: correct and simple, but a lock on every push and pop. None: not allowed by the task.
- `std::sync::mpsc::sync_channel`: a bounded queue that handles the ordering for you. None: no close sibling for teaching the ordering itself.

## In the wild
- Every low-latency queue between one producer and one consumer (market data, order routing) is this pattern, with cache-line padding added.
- Miri is how you catch these in CI without waiting for a rare interleaving.

In a review, check:
- Every atomic `store` that publishes data written by a non-atomic write has `Release` or stronger.
- Every `load` that guards a non-atomic read has `Acquire` or stronger.
