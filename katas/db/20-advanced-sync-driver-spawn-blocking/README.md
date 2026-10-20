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
