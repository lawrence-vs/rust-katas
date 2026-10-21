# 43 advanced: Relaxed is not enough

**Concept:** `Relaxed` makes each atomic indivisible, but it orders nothing between atomics. A flag stored `Relaxed` can be seen before the data written ahead of it. A `Release` store that an `Acquire` load reads makes everything before the store visible after the load.
**Source:** Rust Atomics and Locks (Mara Bos), ch. 3 Memory Ordering, "Happens-Before Relationship" and "Release and Acquire Ordering", https://marabos.nl/atomics/memory-ordering.html
**Fails with:** `reader saw ready but stale data`
**Vault:** your note on happens-before

The library uses loom's atomics so the test can explore every execution, so it only works inside `loom::model`. On x86 this bug almost never shows in a plain stress loop; on ARM (Apple M-series, Graviton) it can. loom finds it on any CPU.

## Task
Make `cargo test` pass without changing the test. Change only `Ordering` arguments in `src/`. No `SeqCst`, no `fence`, no `Mutex` or other lock, no new fields.

## Before you fix it
1. Run `cargo test`. The reader saw `ready == true`. Which values did it read for `price` and `qty`, and why is that allowed?
2. Predict which one operation in `publish` and which one in `read` must change, and to what.

## Answer in the PR
1. Explain the failing execution in terms of happens-before: which store did the reader see, and which stores didn't come with it?
2. Why do the `price`/`qty` loads and stores stay `Relaxed` after your fix? What would `Release` on them have bought?
3. Why does the line after `writer.join()` pass even on the broken code?
4. What would `SeqCst` everywhere cost, and what would it add here?
5. Why would a loop of 1,000,000 runs on CI's x86 machine probably pass the broken code?

## Think further
1. `publish` is called twice, (101, 5) then (102, 7). Can a reader get (102, 5)? What design (a seqlock, or swapping an `Arc` pointer) prevents a torn read?
2. Rewrite the reader with a `Relaxed` load followed by `fence(Acquire)`. When is a fence better than an `Acquire` load?
3. How would you let this library build with std atomics normally and loom atomics under `RUSTFLAGS="--cfg loom"`? What does `check-cfg` need?

## In the wild
- std `Arc`'s `Drop` ([`library/alloc/src/sync.rs`](https://github.com/rust-lang/rust/blob/master/library/alloc/src/sync.rs)): `fetch_sub(1, Release)` then `fence(Acquire)`.
- std `Once`/`OnceLock` ([`library/std/src/sys/sync/once/`](https://github.com/rust-lang/rust/tree/master/library/std/src/sys/sync/once)): Acquire/Release on the state word.
- [tokio](https://github.com/tokio-rs/tokio) and [crossbeam](https://github.com/crossbeam-rs/crossbeam) run their sync primitives under [loom](https://github.com/tokio-rs/loom) with `cfg(loom)`.
- Exchange-style: a market-data thread publishing a book snapshot to strategy threads.

In a review, check:
- A `Relaxed` flag that guards other data.
- `SeqCst` used as "safe by default" with no reason given.
- Lock-free code with no loom or Miri test.
