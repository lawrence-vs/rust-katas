# 41 intermediate: Rc across threads

**Concept:** `thread::spawn` needs a closure that is `Send` and `'static`. `Rc` is neither safe to send nor to share: its count is a plain integer, and two threads changing it at once corrupt it. A thread that may outlive the call needs an owned, atomically counted `Arc`. Threads that are all joined before the function returns can borrow the caller's data inside `std::thread::scope`, with no `Arc` and no copy.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 19 Concurrency: "Fork-Join Parallelism", "Sharing Immutable Data Across Threads", "Thread Safety: Send and Sync", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch19.html; std [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) docs (the book predates it and uses `crossbeam::scope`)
**Fails with:** `E0277`
**Vault:** your note on Send and Sync

## Task
Make `cargo test` pass without changing the test. No `Rc`, `RefCell`, `Mutex` or `unsafe` in `src/`. In `parallel_total`, no `Arc`, `to_vec` or `clone`: its threads borrow `fills`.

## Before you fix it
1. Run `cargo test`. Read the E0277 notes: which trait is missing, on which type, and in which functions?
2. Read the signature of `std::thread::spawn`. Which two bounds does it put on the closure, and which one does each function here really break?

## Answer in the PR
1. Why is `Rc` not `Send`? Name the part of `Rc` that isn't thread-safe and what two threads could do to it.
2. Why does `spawn_audit` need `Arc` and can't use `thread::scope`?
3. Why does `parallel_total` need no `Arc` at all? What does `scope` guarantee that lets a thread borrow `fills`?
4. What did the starting `parallel_total` cost that your version doesn't? Name the allocation and the copy.
5. After `join`, `Arc::strong_count(&fills)` is 1. When was the thread's clone dropped?

## Think further
1. The audit should also append a correction to the fills. What does `Arc<Vec<u64>>` stop you doing, what would you add, and why not `Rc<RefCell<_>>`?
2. `Rc` is `!Sync` as well as `!Send`. What could go wrong if two threads each held a `&Rc<T>`?
3. A worker panics. What does `thread::scope` do when it returns, and what happens to a panic in a `thread::spawn` thread whose `JoinHandle` is dropped?
4. When does spawning a thread per chunk cost more than one loop? What do rayon's `par_iter` and its thread pool change?

## In the wild
- [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) ("`Rc` uses non-atomic reference counting … cannot be sent between threads") and [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) ("Thread Safety").
- [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html), stable since Rust 1.63; it replaced `crossbeam::scope` for borrowing across threads.
- `tokio::spawn` has the same `Send + 'static` bound (kata 17); [rayon](https://github.com/rayon-rs/rayon)'s `join` and `par_iter` borrow, like `scope`.
- Exchange-style: matching threads share a read-only instrument table through `Arc`; an end-of-day reconciliation splits the day's fills across scoped threads.

In a review, check:
- `Rc` in a type that is meant to cross threads, or `unsafe impl Send` added to silence E0277.
- `Arc::new(data.to_vec())` only to spawn threads that are joined before the function returns: use `thread::scope`.
- `Arc<Mutex<_>>` around data no thread writes.
