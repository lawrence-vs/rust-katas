# 189 intermediate: two awaits in a row run one after the other

**Concept:** `.await` suspends the enclosing async fn until that future completes. Two awaits in sequence take the sum of their latencies. To run independent futures at the same time, poll them together with `tokio::join!` (or `JoinSet` / `spawn` when they must be tasks).
**Source:** tokio `join!` macro docs, https://docs.rs/tokio/1.53.2/tokio/macro.join.html; Tokio tutorial, spawning, https://tokio.rs/tokio/tutorial/spawning
**Fails with:** `quotes_run_in_parallel`
**Vault:** your note on concurrency inside one task versus spawned tasks

## Task
Make `cargo test` pass without changing the test. Keep it on one task: no `tokio::spawn`.

## Before you fix it
1. The test reports how long the call took. What is that number, and what would two overlapping 100ms calls give?
2. Where in the function does the second venue's call start?

## Answer in the PR
1. Why does `join!` need no `'static` bound, while `spawn` does? What does each one cost?
2. What happens if one venue's future never completes, under `join!` and under sequential awaits?

## Related concepts
- **`tokio::spawn`:** runs a future as its own task on the runtime, possibly on another thread. Needs `'static` and `Send`.
- **`JoinSet`:** a set of spawned tasks you can await as they finish. Use it when the count is not known up front.

```rust
let handle = tokio::spawn(fut_a);
```
