# 58 advanced: graceful shutdown

**Concept:** A pool owns its tasks. Shutdown signals them, lets in-flight work finish, hands back what never started, and aborts what outlives the grace period, so nothing is lost.
**Source:** Tokio topics, graceful shutdown, https://tokio.rs/tokio/topics/shutdown; tokio `JoinSet` and `watch` docs
**Fails with:** `in-flight settlements were lost`
**Vault:** your note on task ownership and shutdown

## Task
Make `cargo test` pass without changing the tests, `settle` or `Cargo.toml`, and without adding `#[allow]`. Time is paused in every test, so timings are exact.

## Before you fix it
1. Where is job 1 when `shutdown` returns?
2. When the stop signal and a queued job are both ready, which `select!` arm runs?

## Answer in the PR
1. Who owned each worker task in the starting code, and where did jobs 1-3 go?
2. Why does dropping the `Settler`'s own `Sender` not stop the workers in the second test?
3. You picked a signal. What does it do for a worker that starts waiting *after* it was sent? Compare `Notify::notify_waiters` and `tokio_util::sync::CancellationToken`.
4. What would `select!` without `biased;` (or an `is_cancelled` check) do in the last test, and why would that make it fail only sometimes?

## Think further
1. A job aborted halfway through `settle` may have reached the bank. Is it settled, unsettled or unknown? (kata 50)
2. A `submit` races with `shutdown`. Can a job be accepted after the drain and lost?
3. How does `grace` relate to a SIGTERM (`tokio::signal`) and the orchestrator's kill deadline?
4. `unstarted` lives in memory. What makes it survive a crash (an outbox, kata 61)?

## In the wild
- [Tokio topics: graceful shutdown](https://tokio.rs/tokio/topics/shutdown): detect the signal, tell every part, wait for them.
- [tokio `JoinSet`](https://docs.rs/tokio/1.53.2/tokio/task/struct.JoinSet.html): `join_next`, `abort_all`, `shutdown`, and dropping the set aborts its tasks.
- [tokio `mpsc::Receiver::close`](https://docs.rs/tokio/1.53.2/tokio/sync/mpsc/struct.Receiver.html#method.close): stop new sends but keep draining what is queued.
- [tokio-util `CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) and [`TaskTracker`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html): the usual ready-made pieces.
- [axum `serve(...).with_graceful_shutdown`](https://docs.rs/axum/latest/axum/serve/struct.Serve.html#method.with_graceful_shutdown): the same idea for a server.
- Exchange-style: a deploy rolls the withdrawal/settlement service. Detached workers die with the process and leave transfers in an unknown state, or the old pod hangs until the orchestrator kills it.

In a review, check:
- A discarded `JoinHandle`.
- `while let Some(_) = rx.recv()` with no other exit.
- A shutdown with no deadline.
- A `select!` whose arm order matters but has no `biased;`.
- An `abort` with no accounting of what was cut off.
