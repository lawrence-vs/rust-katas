# 18 advanced: a lock held over a slow await

**Concept:** `tokio::sync::Mutex` can be held across `.await`, but that doesn't mean it should be: a fetch under the lock makes every other task wait. Lock for the lookup and the insert, never for the I/O; then a std `Mutex` is enough.
**Source:** Tokio tutorial, shared state (tasks, threads and contention), https://tokio.rs/tokio/tutorial/shared-state
**Fails with:** `gets ran one at a time`
**Vault:** your note on lock scope in async code

## Task
Make `cargo test` pass without changing the test or `fetch`. Time is paused in the test, so it is exact.

## Before you fix it
1. How long is the map locked on a miss?
2. Which steps need the lock: lookup, fetch, insert?

## Answer in the PR
1. Why did ten different keys take 1s, and what do they take now?
2. Your fix lets two tasks fetch the same missing key at once. Is that a bug? (the capstone, kata 31, comes back to this)
3. Why can the fix use `std::sync::Mutex` here, when kata 16 said to keep std locks away from `.await`?

## Think further
1. In the starting code, `fetch` panics or its task is aborted while it holds the lock. What happens to the `tokio::sync::Mutex` and to the tasks waiting on it? Is a std `Mutex` different here?
2. Two tasks fetch the same key and the backend changes in between, so they get different values. Which value stays in the map? Does that matter for a tick size? For a price?
3. Reads far outnumber misses. Would a `RwLock` help when the lock is held for microseconds? What about a sharded map like `DashMap`, and what happens if you keep one of its references across an `.await`?

## In the wild
- [tokio `Mutex`, "Which kind of mutex should you use?"](https://docs.rs/tokio/1.53.2/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use): what the async mutex is actually for.
- [tokio `RwLock`](https://docs.rs/tokio/1.53.2/tokio/sync/struct.RwLock.html): read the fairness note. A queued writer blocks new readers.
- [dashmap `DashMap`](https://docs.rs/dashmap/latest/dashmap/struct.DashMap.html): read the "Locking behaviour" notes on its methods.
- [Clippy `await_holding_lock`](https://clippy.rust-lang.org/master/index.html#await_holding_lock): it does not fire for `tokio::sync::Mutex`, so this kata's bug passes clippy. Reviewers have to catch it.
- [moka `future::Cache`](https://docs.rs/moka/0.12.16/moka/future/struct.Cache.html): a production async cache. Kata 31 builds a small one.
- Exchange-style: a reference-data cache (instruments, fee tiers) in front of a database. One lock held over the query puts every order behind the slowest miss, and that shows up as p99 latency.

In a review, check:
- A `tokio::sync::Mutex` guard alive across a database, HTTP or RPC `.await`. No lint flags this.
- `tokio::sync::Mutex` where no `.await` ever happens under the lock.
- A lock taken on the first line of a function and released only at `return`.
- A miss path that awaits the backend with no deadline (kata 22).
