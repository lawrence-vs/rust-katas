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
