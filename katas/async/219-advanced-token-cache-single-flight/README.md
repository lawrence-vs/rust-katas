# 219 advanced: a token cache that fetches once

**Concept:** A cache that must call upstream once per cold burst ("single flight") has to keep the first caller's fetch visible to the others. Checking, releasing and re-taking the lock leaves a gap in which every concurrent caller also sees "empty" and fetches.
**Source:** kraken-cli `crates/kraken-ws/src/token.rs:89-152` (https://github.com/krakenfx/kraken-cli)
**Fails with:** `upstream fetched`
**Vault:** your note on single-flight and async locks

## Task
Make `cargo test` pass without changing the tests. Keep one upstream fetch per cold burst. Don't add a second cache. Don't use `std::sync::Mutex` across the await.

## Related concepts

### `tokio::sync::OnceCell::get_or_init`
An async cell that runs its init future at most once and hands the result to every waiter. Use it for lazily built, never-refreshed values.
```rust
static CFG: tokio::sync::OnceCell<u32> = tokio::sync::OnceCell::const_new();
let v = CFG.get_or_init(|| async { 7 }).await;
```

### `std::sync::OnceLock`
A synchronous once-cell for values computed without awaiting. Use it for global config built at first use.
```rust
static NAME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
let n = NAME.get_or_init(|| "kraken".to_string());
```

### `tokio::sync::Mutex` vs `std::sync::Mutex`
The std mutex blocks a thread and its guard is not meant to live across `.await`; the tokio mutex has an async `lock()` and a guard that may be held across `.await`.
```rust
let m = tokio::sync::Mutex::new(0);
let mut g = m.lock().await;
*g += 1;
```

## Before you fix it
1. Read the failing count. How many callers saw an empty cache, and what let them all see it?
2. Where exactly does the first caller stop holding the lock, and what runs during the 50ms that follows?

## Answer in the PR
1. What is the cost of holding the lock across the fetch, and how would a timeout bound it?
2. Why is `std::sync::Mutex` wrong here?
3. Why does `get` take a boxed future instead of a plain generic future-returning closure?

## Think further
1. If the fetch fails, should the next waiter retry or share the error? What does your code do?
2. When the token expires, how would you refresh it without blocking readers of a still-valid token?

## In the wild
- [tokio `Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html): see "Which kind of mutex should you use?" and holding a guard across `.await`.
- [tokio `OnceCell`](https://docs.rs/tokio/latest/tokio/sync/struct.OnceCell.html): an async once-initialised cell.
- Exchange-style: many websocket tasks need the same auth token at connect time; one REST call should serve them all.

In a review, check:
- A lock released before an `.await` whose result is stored under a second lock.
- A cache read and its write in two separate critical sections.
