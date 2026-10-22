# 56 intermediate: rayon inside async

**Concept:** `par_iter` blocks its caller until rayon finishes. On a tokio worker that stops every other task, so hand the work to another thread (`spawn_blocking`, or `rayon::spawn` + `oneshot`) and await the answer.
**Source:** tokio docs, [CPU-bound tasks and blocking code](https://docs.rs/tokio/1.53.2/tokio/index.html#cpu-bound-tasks-and-blocking-code); rayon [`spawn`](https://docs.rs/rayon/1.12.0/rayon/fn.spawn.html) docs; Alice Ryhl, "Async: What is blocking?" (its rayon section); kata 20.
**Fails with:** `the timer task stalled while the book was valued`
**Vault:** your note on CPU work in async code

## Task
Make `cargo test` pass without changing the tests, `level_value` or `Cargo.toml`. Keep the levels valued in parallel. No `unwrap`, `expect` or `panic!` in `src/`.

## Before you fix it
1. Which test passes already, and why?
2. Which thread runs `level_value`, and which thread waits?

## Answer in the PR
1. rayon already ran the levels on its own threads. Why did the timer still stall?
2. `spawn_blocking` or `rayon::spawn` + `oneshot`: in your fix, which thread waits for rayon, and what does the other option cost?
3. Why must the closure own `levels`? What does the compiler say if `book_value` took `&[Level]`?
4. When does your fix return `WorkerLost`? Walk through a panic inside `level_value`.

## Think further
1. 1,000 requests arrive at once. Each one parks a blocking-pool thread on rayon. What bounds them (a `Semaphore`, kata 21)?
2. The caller times out (kata 22). Does the rayon work stop? What would a cancel flag checked between levels cost?
3. With no `panic_handler` set, a panic inside `rayon::spawn` aborts the process. Read the docs. How does that change your choice of fix?
4. Why does this kata measure real time instead of using `start_paused`?
5. Each level takes 20 µs, not 300 ms. Is hopping threads still worth it?

## In the wild
- An exchange REST handler that builds a full depth snapshot, or recomputes margin across all accounts on a price tick.
- Risk or matching engines pinned to their own threads and fed by channels.
- The tokio docs section above.
- rayon `spawn` and `ThreadPoolBuilder::panic_handler`.
- Alice Ryhl's post.

In a review, check:
- `par_iter`, `rayon::join`/`scope`, hashing or large JSON work directly inside an `async fn`.
- `spawn_blocking` around rayon with no bound on concurrent requests.
- `JoinError` or `RecvError` unwrapped.
- `rayon::spawn` with no panic handler.
