# 16 beginner: a std MutexGuard across `.await`

**Concept:** A `std::sync::MutexGuard` held across `.await` makes the future non-`Send`, so `tokio::spawn` rejects it, and if it did run it could block a worker. Keep std locks short: lock, change, drop, then await.
**Source:** Tokio tutorial, shared state (holding a MutexGuard across an .await), https://tokio.rs/tokio/tutorial/shared-state
**Fails with:** `future cannot be sent between threads safely`
**Vault:** your note on locks and await points

## Task
Make `cargo test` pass without changing the test. Keep `std::sync::Mutex`: the critical section is short and synchronous.

## Before you fix it
1. Read the error. Which value does it say is held across which `.await`?
2. Why does `tokio::spawn` need `Send`?

## Answer in the PR
1. Why is a guard held across `.await` dangerous even on one thread?
2. Switching to `tokio::sync::Mutex` also compiles. Why is std still the right lock here?
3. When is `tokio::sync::Mutex` the right lock?
