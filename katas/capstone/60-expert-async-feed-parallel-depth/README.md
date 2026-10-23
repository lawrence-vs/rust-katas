# 60 expert: capstone: an async feed with parallel depth

**Concept:** A pipeline that reads JSON from the network must validate it into typed values at the edge, bound its queue so a fast producer waits, keep CPU-heavy rayon work off the async runtime with `spawn_blocking`, and drain the queue before it shuts down.
**Source:** tokio `spawn_blocking` and `mpsc::channel` docs, https://docs.rs/tokio/1.53.2/tokio/; tokio graceful shutdown topic, https://tokio.rs/tokio/topics/shutdown; serde container attributes (`deny_unknown_fields`), https://serde.rs/container-attrs.html; rayon `par_iter` docs, https://docs.rs/rayon/1.12.0/rayon/; own design (exchange market-data take-home)
**Fails with:** `an invalid update was accepted`
**Vault:** your note on async pipelines

## Task
A feed takes one JSON order-book update per `submit`, rebuilds the book per symbol, recomputes depth and VWAP for the touched symbols in parallel with rayon, and hands each `Depth` to a `publish` closure. Make the stages pass in order without changing the tests or any `pub` signature.
- No `unwrap`, `expect` or `panic!` in `src/`.
- No `f64`.
- No crates beyond `Cargo.toml`.

The wire format is one object per `submit`: `{"symbol":"BTC-USD","seq":7,"side":"bid","price":100,"qty":5}`. All five fields are required and unknown fields are rejected. `price` (ticks, `1..=MAX_PRICE`) and `qty` (lots, `0..=MAX_QTY`, 0 removes the level) are JSON integers. A symbol is 1 to 16 characters of `A-Z`, `0-9` and `-`.

## Stages

Work in order. After each stage, write down your **Observation** (what the test showed), a **Question** it raised, and the **Insight** you'd keep. Put all three in the PR.

1. `stage1_invalid_updates_are_rejected`: input from the network is data, not a promise.
2. `stage2_the_recompute_does_not_block_the_runtime`: a runtime thread is for waiting, not for crunching.
3. `stage3_a_full_queue_pushes_back`: a queue with no bound is a memory leak with extra steps.
4. `stage4_shutdown_publishes_every_update`: accepted means published.

## Before you fix it
0. Write `DESIGN.md` in this directory (at most one page) before you touch the code. Cover the path of one update from `submit` to `publish`, which thread runs each step, what bounds memory, what `shutdown` guarantees, and your capacity choice.
1. Run `cargo test`. Which test passes today, and why?
2. Run `cargo test stage2`. Why does it take 2 seconds?

## Answer in the PR
1. What changed in `DESIGN.md` after the tests ran?
2. Stage 1: what did `as u64` do to `-5` and `1.5`? Why derive a wire type and convert with `TryFrom` instead of reading a `Value`?
3. Stage 2: which thread runs `par_iter` now? What had to be owned to cross into the closure (cloning the touched books vs moving the map in and back), and what does each option cost?
4. Stage 3: what does a full queue do to the submitter now? Why `send().await` here and not kata 21's `try_send`? Where could an unbounded buffer still hide (hint: the drain loop)?
5. Stage 4: what does `shutdown` guarantee now? What does the caller see if the pipeline panics? What bounds how long shutdown takes?
6. Which earlier kata does each stage come from?

## Think further
1. A slow publisher: should the pipeline conflate to the latest depth per symbol (`tokio::sync::watch`), or block?
2. A shutdown deadline, and a `CancellationToken` for when upstream won't stop.
3. Bounding the batch (`max_batch`), and fairness when one symbol is hot.
4. Out-of-order or duplicate `seq` per symbol (kata 52).
5. When is rayon not worth it for three symbols, and how would you measure that?
6. `capacity` comes from config: why is it a `NonZeroUsize`?

## In the wild
- tokio's [graceful shutdown](https://tokio.rs/tokio/topics/shutdown) topic: signal, wait for tasks to finish, then exit.
- tokio [`spawn_blocking`](https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn_blocking.html) and Alice Ryhl's "Async: What is blocking?" (the rayon plus `oneshot` bridge for CPU-bound work).
- serde [`deny_unknown_fields`](https://serde.rs/container-attrs.html#deny_unknown_fields): reject input you did not design for.
- Exchange depth feeds that conflate per symbol: a slow consumer gets the latest book, not every step.
- Market-data handlers choose between backpressure and drop-with-gap, and say which in their docs.

In a review, check:
- `Value` indexing or `as` casts on network input.
- `par_iter` or `std::thread::sleep` inside an `async fn`.
- An `unbounded_channel` fed from the network.
- `abort()` or a dropped `JoinHandle` on shutdown.
- An unwrapped `JoinError`.
