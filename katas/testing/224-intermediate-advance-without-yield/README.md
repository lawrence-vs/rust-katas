# 224 intermediate: a clock advanced without giving the task a turn

**Concept:** under `start_paused = true`, `tokio::time::advance` moves the virtual clock and wakes timers, but a woken task only runs when the runtime polls it. A helper that steps the clock without yielding gives a task that needs several turns no chance to finish.
**Source:** [`tokio::time::advance`](https://docs.rs/tokio/latest/tokio/time/fn.advance.html); [`tokio::task::yield_now`](https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html); the `advance_until` helper in Kraken's `connection_tests.rs` (MIT) yields 20 times, then advances 100 ms, for up to 200 rounds.
**Fails with:** `condition never met`
**Vault:** your note on paused time in tests

## Task
Make `cargo test` pass without changing the tests. Keep the signature of `advance_until`. The helper must give the runtime a turn between clock steps; do not sleep for real time and do not add dependencies.

## Related concepts
### `tokio::time::sleep`
Waits on the runtime clock; under paused time it completes only when something advances the clock.
```rust
tokio::time::sleep(std::time::Duration::from_secs(1)).await;
```

### `tokio::time::pause`
Freezes the clock in a running runtime, which `start_paused = true` does from the first line of a test.
```rust
tokio::time::pause();
```

### `tokio::sync::Notify`
Wakes a waiter on an event rather than on a deadline; often a better test signal than polling a flag.
```rust
let n = tokio::sync::Notify::new();
n.notify_one();
```

## Before you fix it
1. The first test fails and the second passes. What does the second one never need from the runtime?
2. After the timer fires, how many polls does the spawned task need before it sets the flag?

## Answer in the PR
1. What does `advance` do to the clock and to timers, and what does it not do for the tasks they wake?
2. Why does a spawned task need a poll before it can observe anything, even after its timer has fired?
3. What would a fixed count of yields risk, and what keeps the helper bounded?

## Think further
1. Replace the polled flag with a `Notify` or channel. What does the test lose and gain?
2. Why does `advance` followed by the check sometimes pass for tasks that need only one turn?

## In the wild
- [tokio `time::advance`](https://docs.rs/tokio/latest/tokio/time/fn.advance.html): the docs describe the single yield it performs after moving the clock.
- [tokio `task::yield_now`](https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html): hands the thread back so other ready tasks run.
