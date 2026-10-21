# 20 advanced: a sync driver on the async runtime

**Concept:** A synchronous driver (rusqlite, diesel) blocks its thread. On a tokio worker that stalls every task on that worker. `spawn_blocking` moves the call to a thread pool meant for blocking.
**Source:** tokio `spawn_blocking` docs, https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn_blocking.html
**Fails with:** `lookups_do_not_block_the_runtime`
**Vault:** your note on blocking in async code

## Task
Make `cargo test` pass without changing the test or `legacy_lookup`. The test runs a single-threaded runtime.

## Before you fix it
1. The test runs four lookups on a current-thread runtime. Why do they take 400ms?
2. Which thread does `spawn_blocking` run its closure on?

## Answer in the PR
1. Why does a blocking call hurt tasks that have nothing to do with the database?
2. Why does the closure need `move`, and what must it own?
3. When would you switch to an async driver (sqlx) instead?

## Think further
1. 10,000 lookups arrive at once. Tokio's blocking pool has a thread limit (`max_blocking_threads`). What happens to lookup 10,000, and what bounds memory while it waits?
2. The caller's future is dropped (the client disconnects, or a timeout fires) while the driver call is running. Does the database work stop? Would your answer worry you more for a write than for a lookup?
3. `tokio::task::block_in_place` also exists. Would it work on the test's runtime? Read its docs before you answer.
4. The driver call takes 2 µs, not 100 ms. Is moving it to another thread still worth it? Where would you draw the line, and how would you measure it?

## In the wild
- [tokio `spawn_blocking`](https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn_blocking.html): read the sections on what can't be cancelled and when to use a plain `std::thread` instead.
- [tokio `block_in_place`](https://docs.rs/tokio/1.53.2/tokio/task/fn.block_in_place.html): the other tool, and the runtime it refuses to run on.
- [tokio runtime `Builder::max_blocking_threads`](https://docs.rs/tokio/1.53.2/tokio/runtime/struct.Builder.html#method.max_blocking_threads): the blocking pool's size limit and its default.
- [tokio `fs`](https://docs.rs/tokio/1.53.2/tokio/fs/index.html): the module docs say how tokio does file I/O on top of a blocking OS API.
- [Alice Ryhl, "Async: What is blocking?"](https://ryhl.io/blog/async-what-is-blocking/): a rule of thumb for how long code may run between `.await`s.
- [sqlx-sqlite connection `worker.rs`](https://github.com/transact-rs/sqlx/blob/v0.8.6/sqlx-sqlite/src/connection/worker.rs): SQLite's C library is synchronous. How does an "async" driver run it?
- [deadpool-diesel](https://docs.rs/crate/deadpool-diesel/latest): pooling a synchronous driver (diesel) for async code.
- Exchange-style: [LMAX architecture](https://martinfowler.com/articles/lmax.html): the matching engine runs on its own thread, fed by queues, not shared with I/O tasks.

In a review, check:
- `std::fs`, `std::thread::sleep`, rusqlite or diesel calls, password hashing or large parsing done directly inside an `async fn`.
- `spawn_blocking` around a loop that never ends; that needs its own thread.
- Unbounded `spawn_blocking` calls driven by network input, with nothing limiting how many are queued.
- `Handle::block_on` or `futures::executor::block_on` called from async code.
- The `JoinHandle`'s `JoinError` unwrapped, so a driver panic becomes a handler panic.
