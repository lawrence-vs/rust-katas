# 234 intermediate: a future that is built but never awaited

**Concept:** calling an `async fn` only builds a future; its body runs when something polls it (`.await`, or an executor such as `tokio::spawn`). A future dropped without a poll did nothing. Scenario (not a reported incident): a gateway calls `send_heartbeat()`, forgets `.await`, nothing is sent and nothing errors.
**Source:** Rust async book, https://rust-lang.github.io/async-book/; Futures Explained in 200 Lines of Rust, "Futures", https://cfsamson.gitbook.io/explained-futures/
**Fails with:** `started_flag_set_after_run`
**Vault:** your note on futures being lazy: building is not running

## Task
Make `cargo test` pass without changing the test. Keep `start_session` private and `run(&self)` as it is. No `tokio::spawn`.

## Before you fix it
1. `cargo build -p kata-async-234`: which warning appears, and on which line?
2. Put a `println!` inside `start_session`. Does it print?

## Answer in the PR
1. Where does the body of an `async fn` start running: at the call, at the first poll, or at the first `.await` inside it?
2. Why is the future type `#[must_use]`, and why is this a warning and not an error?
3. Why can't `tokio::spawn` take this future as `run` is written?

## Related concepts
### `tokio::spawn`
Runs a future as its own task on the runtime, possibly on another thread. Its main use is background work that outlives the caller. Needs `'static` and `Send`.
```rust
let handle = tokio::spawn(async { 1 });
```
### `tokio::join!`
Polls several futures together on the current task and waits for all of them. Its main use is running independent calls concurrently without spawning.
```rust
let (a, b) = tokio::join!(fut_a, fut_b);
```
### `#[must_use]`
Attribute that makes the compiler warn when a value of that type is ignored. Its main use is marking results and futures that do nothing unless used.
```rust
#[must_use]
fn checked() -> bool { true }
```

## Read further
- **Read:** async book, "Executors and System IO" (futures are lazy), https://rust-lang.github.io/async-book/; Futures Explained in 200 Lines of Rust, "Futures", https://cfsamson.gitbook.io/explained-futures/
- **Learn:** a future is a state machine that advances only when polled; `.await` polls it from the caller's task.
- **Gap:** the sources state that futures are lazy but show no future that is built and dropped without running; the test is that example.
- **Next:** kata 66 (hand-written future), kata 189 (sequential awaits).

## Think further
1. Why does the compiler warn (`#[must_use]`)?
2. What changes if the future is passed to `tokio::spawn`, and what would `run` have to own?
3. Construction, first poll or first `.await`: where exactly does the body start?

## In the wild
- [std `Future`](https://doc.rust-lang.org/std/future/trait.Future.html): `poll` is the only way a future makes progress.
- [rustc `unused_must_use`](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unused-must-use): the warning this kata triggers.
- [clippy `let_underscore_future`](https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_future): the same bug hidden behind `let _ =`.
- In a review, check: a call to an `async fn` or a future-returning fn with no `.await`, `spawn` or `join`; `let _ = some_async_call();`.
