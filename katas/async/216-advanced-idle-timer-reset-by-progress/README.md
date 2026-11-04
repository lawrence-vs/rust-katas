# 216 advanced: idle timer that resets only on progress

**Concept:** An idle timeout must be one timer that survives loop iterations. A `sleep(idle)` built inside the loop restarts on every pass, so a heartbeat tick that merely re-enters the loop keeps pushing the deadline away. Hold one pinned `Sleep` outside the loop and `reset` it only when real progress happens.
**Source:** kraken-cli `crates/kraken-ws/src/connection.rs:516-544` (MIT, https://github.com/krakenfx/kraken-cli); reimplemented here.
**Fails with:** `idle never fired`
**Vault:** your note on cancel-safe select loops

## Task
Make `cargo test` pass without changing the test. The idle deadline must not be recreated per loop pass.

## Related concepts
None: no close siblings.

## Before you fix it
1. With a silent peer, how many times does the loop body run before 1s of idle should have elapsed, and what does each pass do to the idle timer?
2. Which `select!` branches are progress and which are not?

## Answer in the PR
1. Why does a timer created per iteration never fire when another branch wakes the loop more often than the timeout?
2. What makes a heartbeat not progress, and where in the code is that decision visible?
3. What is `tokio::pin!` for here, and why can't you `select!` on a `Sleep` you keep across iterations without it?

## Think further
1. `select!` drops the losing futures each pass. Which branch futures here are cancel-safe, and why does that matter for `recv()`?
2. What should the deadline do if a frame arrives in the same instant the timer fires?

## In the wild
- [`tokio::select!`](https://docs.rs/tokio/latest/tokio/macro.select.html): branch cancellation and the loop-with-pinned-future pattern.
- [`tokio::time::Sleep`](https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html): `reset` moves an existing timer's deadline.
