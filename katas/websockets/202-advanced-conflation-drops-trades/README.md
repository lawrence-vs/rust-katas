# 202 advanced: conflation drops trades

**Concept:** Conflation keeps only the latest state per key, so it is safe for a book level (the new qty replaces the old) and never for a trade (each fill happened). Conflate per (symbol, side, price), keep each message's seq, and when the buffer is still full tell the client to resnapshot instead of dropping messages silently.
**Source:** own design (an exchange market-data fan-out to slow clients); std [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html); katas 25 and 52.
**Fails with:** `trades are never conflated`
**Vault:** your note on slow consumers and conflation

## Why
A slow client can't take every update, so the server has to choose what to merge.
Merging the wrong things loses fills, or merges unrelated levels into a wrong book.
Dropping silently leaves a gap the client can't see.

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect` or `panic!` in `src/`. std only. Change only `push`.

## Before you fix it
1. Which tests already pass, and why?
2. In `trades_are_never_conflated`, what does `drain` return, and which line removed the rest?

## Related concepts
- `tokio::sync::watch`: a single-slot channel; receivers see only the latest value. Use for config or a ticker.
  ```rust
  let (tx, rx) = tokio::sync::watch::channel(0);
  tx.send(5).ok();
  let v = *rx.borrow();
  ```
- `tokio::sync::broadcast` lag: a ring buffer that reports `Lagged(n)` to a slow receiver. Use for fan-out where skipping is allowed.
  ```rust
  match rx.recv().await {
      Err(RecvError::Lagged(n)) => { /* n messages skipped */ }
      // ...
  }
  ```
- `std::sync::mpsc::sync_channel`: a bounded channel whose `send` blocks when full. Use for backpressure on a producer you own.
  ```rust
  let (tx, rx) = std::sync::mpsc::sync_channel::<u64>(2);
  tx.try_send(1).ok();
  ```

## Answer in the PR
1. Walk through the starting code on the trades test. Which fills did the client never see, and what would a P&L or a chart built from the stream show?
2. Why is latest-wins safe for a book level and never for a trade? Name the difference between state and event.
3. Why does a conflated update take its new seq and move to the back instead of keeping its old place? What does the client's seq check see either way?
4. Full buffer: why a marker instead of dropping the oldest? What does the client do on `Resnapshot { seq }`, and which deltas does it skip afterwards (kata 52)?

## Think further
1. Conflate on push (this kata) vs mark levels dirty and read the book at send time: which bounds memory better?
2. `retain` scans the queue on every push. Index by key: what breaks when `VecDeque` positions shift?
3. A resnapshot also lost trades. How does the client fetch the fills it missed since `seq`?
4. Resnapshot one symbol instead of the whole buffer: what must the marker carry?
5. A client that resnapshots every second: when should the server disconnect it (kata 25)?

## In the wild
- tokio `watch` (latest value only)
- Kafka log compaction (`cleanup.policy=compact`, latest record per key)
- Redis `client-output-buffer-limit pubsub` (disconnects a slow subscriber)
- Exchange feeds offering a conflated book channel beside an unconflated trades channel

In a review, check:
- conflation keyed on symbol only
- trades or fills behind a latest-wins buffer
- `pop_front` on full with no signal to the client
- a merged update that keeps its old seq
