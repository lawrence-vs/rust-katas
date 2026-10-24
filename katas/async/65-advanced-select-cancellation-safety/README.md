# 65 advanced: a frame lost to select!

**Concept:** `select!` drops the futures that lose. A read that has buffered half a frame inside its future takes that half with it, so a branch in a loop must be cancel-safe or survive the loop.
**Source:** Rust for Rustaceans, ch 8 (Asynchronous Programming); tokio `select!` docs, "Cancellation safety"
**Fails with:** `a heartbeat mid-frame lost bytes`
**Vault:** your note on cancellation safety

## Task
Make `cargo test` pass without changing the tests or `run`'s signature. Keep both branches in the `select!` and the `Interval` heartbeat. std + tokio only (no `tokio-util`, no new crates). `biased;` doesn't count as a fix. No `unwrap`/`expect`/`panic!` in `src/`. Time is paused in every test, so timings are exact.

## Before you fix it
1. Which two tests already pass, and why doesn't either of them catch the bug?
2. In the failing test, where are the bytes `he` at t=1000ms, and where are they at t=1001ms?

## Answer in the PR
1. Explain the error message: where does 1819045632 come from?
2. Which future did `select!` drop, and why does the heartbeat branch not lose anything?
3. Which fix did you pick (buffer in a struct, or a future kept alive across loop turns)? What does the other one cost, and what borrow problem do you hit if you `Box::pin(read_frame(&mut reader))` and try to `.set(...)` a new one?
4. Does your fix change what happens when the stream ends mid-frame?

## Think further
1. `tokio::time::timeout(d, read_exact(..))` has the same bug. Why?
2. Why is `mpsc::Receiver::recv` cancel-safe and `read_exact` not?
3. The starting code treats a truncated header as a clean EOF, and that's a bug too.
4. A reader task that owns the socket and sends frames over a channel: what does that cost in shutdown (kata 58)?
5. `select!` on `&mut fut` vs a fresh future each turn.

## In the wild
- [tokio `select!`](https://docs.rs/tokio/1.53.2/tokio/macro.select.html), "Cancellation safety": which methods are safe to drop and retry.
- [`AsyncReadExt::read_exact`](https://docs.rs/tokio/1.53.2/tokio/io/trait.AsyncReadExt.html#method.read_exact): documented as not cancellation safe.
- [mini-redis `Connection::read_frame`](https://github.com/tokio-rs/mini-redis/blob/master/src/connection.rs): a buffer that lives in the connection, not the future.
- [`tokio_util::codec::FramedRead`](https://docs.rs/tokio-util/latest/tokio_util/codec/struct.FramedRead.html): what this kata hand-rolls.
- Rust for Rustaceans, ch 8.
- Exchange-style: a market-data or FIX session loop with a heartbeat branch that misframes the stream once a day under load.

In a review, check:
- `read_exact`/`read_to_end`/`read_line` inside a `select!` in a loop.
- A `timeout` around a multi-step read.
- State built up inside a future that a `select!` can drop.
