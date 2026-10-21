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

## Think further
1. One task panics. What does its `h.await` return, and what happens to every other length when the loop calls `unwrap()` on it?
2. `words` has a million entries. How many tasks run at once, and what does each one cost? Where would you put a limit? (kata 21)
3. The caller gives up halfway (a `timeout` around `word_lengths`, or the request is dropped). Do the spawned tasks stop? Would the answer change if they lived in a `JoinSet`?

## In the wild
- [tokio `spawn`](https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn.html): read the bounds on `F`. `'static` is the reason a task can't borrow the caller's locals.
- [Tokio tutorial, Spawning](https://tokio.rs/tokio/tutorial/spawning): the sections on the `'static` bound and the `Send` bound.
- [tokio `JoinHandle`](https://docs.rs/tokio/1.53.2/tokio/task/struct.JoinHandle.html) and [`JoinSet`](https://docs.rs/tokio/1.53.2/tokio/task/struct.JoinSet.html): compare what happens to the tasks when each one is dropped.
- [std `thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html): threads that are always joined can borrow. Ask why tokio has no safe version of this for tasks.
- Exchange-style: a gateway fans a request out to several matching shards and gathers their replies. Fills from many connections flow over an `mpsc` into one task that owns the book.

In a review, check:
- `Arc<Mutex<Vec<_>>>` whose only job is collecting results from tasks.
- A `JoinHandle` dropped or bound to `let _ =`: the task's panic or error disappears unseen.
- `handle.await.unwrap()` in a loop, so one failed task throws away all the results.
- A spawn per item with no limit on how many items can arrive.
