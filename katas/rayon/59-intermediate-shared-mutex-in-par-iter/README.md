# 59 rayon (intermediate): a shared Mutex inside par_iter

**Concept:** Per-item results come out of `map` + `collect`, which keeps input order and takes no lock. An aggregate comes out of `fold`/`reduce`. A shared `Mutex` turns the parallel loop back into a queue for one lock, and keeps whatever order the threads finish in.
**Source:** rayon docs for [`ParallelIterator`](https://docs.rs/rayon/1.12.0/rayon/iter/trait.ParallelIterator.html) (`for_each`, `collect`, `fold`, `reduce`, `try_reduce`) and [`IndexedParallelIterator`](https://docs.rs/rayon/1.12.0/rayon/iter/trait.IndexedParallelIterator.html); the rayon FAQ; katas 35 and 41.
**Fails with:** `results come back in input order`
**Vault:** your note on data parallelism without shared state

## Task
Make `cargo test` pass without changing the tests, and keep the public API. No `Mutex`, `RwLock`, `lock`, `sort*`, `unwrap` or `expect` in `src/`, and no new crates.

## Before you fix it
1. Run `cargo test` and say why only the order test fails.
2. Say why the 500 ms sleep on position 0 makes it fail every time.

## Answer in the PR
1. Which thread pushed first, and why does `for_each` promise no order?
2. How does `collect` into a `Vec` keep the input order without a lock? Answer in terms of indexed iterators and each split writing its own slot.
3. What was the `Arc` for, and why didn't `for_each` need it?
4. For `total_exposure`, what are your identity and your operation, and why must the operation be associative?
5. `fold` + `reduce`, `try_reduce` or `map().sum()`: which did you pick? What does `sum` do on overflow in debug builds compared with release builds?

## Think further
1. How would you measure the contention a shared lock causes?
2. `par_chunks` or one task per item: when does each win?
3. When would a `Mutex` per shard be fine?
4. What does `collect_into_vec` give you that `collect` doesn't?
5. How do `unzip` and `partition` produce two outputs in one pass?
6. Why is the closure bound `Sync` and not `Send`?

## In the wild
- End-of-day mark-to-market of every account.
- Margin and risk recalculation across a book.
- Parallel reconciliation of fills against ledger entries.

In a review, check:
- `lock()` inside `par_iter`.
- A `for_each` that pushes into a shared collection.
- A `sort` after a parallel collect that only hides lost order.
