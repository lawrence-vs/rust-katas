# 17 intermediate: spawn, then send results back

**Concept:** `tokio::spawn` needs a `'static` future: a task can't borrow the caller's locals. Give each task what it owns (`move`) and get results back through its `JoinHandle` or an `mpsc` channel, not through a shared Vec.
**Source:** Tokio tutorial, channels, https://tokio.rs/tokio/tutorial/channels; Network Programming With Rust (Chanda), tokio client, https://readwise.io/open/900600971
**Fails with:** `E0373`
**Vault:** your note on message passing between tasks

## Task
Make `cargo test` pass without changing the test. No `Mutex` and no `Arc`.

## Before you fix it
1. Read E0373. Which variable does the async block borrow, and why may it outlive the function?
2. After `move`, which errors remain, and what do they say about sharing `lengths`?

## Answer in the PR
1. Fix A: an `mpsc` channel. Fix B: return the length from each task and collect `JoinHandle` results. Which did you pick, and when is the other better?
2. Why does the channel version need `drop(tx)` before reading?
3. What would `Arc<Mutex<Vec<_>>>` cost here?
