# 221 advanced: a drop guard that cancels its worker too early

**Concept:** `CancellationToken::drop_guard()` returns a `DropGuard` that cancels the token when it is dropped. `let _ = guard;` drops the value at once; `let _guard = guard;` keeps it to the end of the scope. A guard that is meant to tie a task's life to its owner must be stored in the owner.
**Source:** kraken-cli `crates/kraken-ws/src/connection.rs:100-117` (MIT; the shape is reimplemented here), and [`DropGuard`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.DropGuard.html)
**Fails with:** `worker stopped while its owner was alive`
**Vault:** your note on RAII guards: cleanup tied to a value's scope

## Task
Make `cargo test` pass without changing the test. Keep the guard alive for as long as the `Worker` lives. Do not remove the cancellation: dropping the `Worker` must still stop the task.

## Related concepts
### `CancellationToken::cancel`
Cancels the token explicitly, from any clone. Use it when shutdown is a deliberate step rather than a side effect of scope.
```rust
let token = CancellationToken::new();
let t = token.clone();
tokio::spawn(async move { t.cancelled().await });
token.cancel();
```

### `JoinHandle::abort`
Stops a task at its next `.await` without its cooperation. Use it when the task has no cleanup to run.
```rust
let h = tokio::spawn(async { std::future::pending::<()>().await });
h.abort();
```

### `scopeguard::defer!`
Runs a closure when the enclosing scope ends, the general form of a drop guard. Use it for cleanup on every exit path.
```rust
scopeguard::defer! { println!("left scope"); }
```

## Before you fix it
1. Which line of the test fails, and what had happened to the worker before the clock advanced?
2. At which point is the `DropGuard` dropped in `start`?

## Answer in the PR
1. What does `_` versus `_name` do to a value's lifetime in a `let`?
2. Why is the guard stored in `Worker`?
3. What would a plain `cancel()` call change?

## Think further
1. `Worker` is moved into another struct or sent to another task: when does the guard drop then?
2. Dropping the guard cancels but does not wait. How would you wait for the task to finish, and why can't `Drop` do it with `.await`?

## In the wild
- [tokio-util `DropGuard`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.DropGuard.html): cancels the token on drop; `disarm()` takes the token back.
- [Rust Reference: drop scopes](https://doc.rust-lang.org/reference/destructors.html#drop-scopes): when a value is dropped, including the `_` pattern versus a named binding.

In a review, check:
- `let _ = ...` on a guard, lock guard or permit: it drops immediately.
- A background task spawned with no owner that stops it.
