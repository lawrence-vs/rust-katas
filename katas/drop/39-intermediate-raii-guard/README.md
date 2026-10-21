# 39 intermediate: an RAII guard, not a manual release

**Concept:** Cleanup that the caller has to remember gets skipped on the first early `return`, `?` or panic. Put it in `Drop` on a value that owns, or borrows, the resource. Rust drops every live local on every way out of a scope, unwinding included, so the cleanup can't be forgotten. That value is a guard (RAII: the resource is released when its owner goes away). A guard that borrows what it cleans up is a struct holding a reference, so it carries a lifetime.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 13 Utility Traits, "Drop", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch13.html
**Fails with:** `a_bad_quantity_frees_its_slot`
**Vault:** your note on Drop and RAII

## Task
Make `cargo test` pass without changing the test. Nothing outside `Drop` may decrement `in_flight`: no public `release`, no extra `release()` calls before each `return`. No crates (`scopeguard`), no `unsafe`, no `mem::forget`.

## Before you fix it
1. Run `cargo test`. Four tests fail. For each one, which exit from `submit`, or which line in the test, kept the slot?
2. `a_held_slot_counts_until_dropped` calls `drop(first)`. What type is `first` now, and what does the compiler warning about that `drop` tell you?

## Answer in the PR
1. List every way the starting `submit` could leave. Which of them skipped `release()`?
2. Why does `Drop` also run when a panic unwinds through `submit`? What happens to the locals of each frame on the way up?
3. Your guard reaches `in_flight`. What does it borrow, which lifetime does its type carry, and what would the compiler stop if a guard tried to outlive its `Gateway`?
4. In `submit`, what is the difference between `let _ = self.reserve()?;` and `let _slot = self.reserve()?;`? What would the first do to the limit?
5. Why not just add `self.release()` before each `return`? Name the path that would still leak, and the one a future change would add.

## Think further
1. `std::mem::forget(slot)` is safe Rust. What happens to the count, and why does Rust treat a leak as safe rather than undefined behaviour?
2. A `Drop` impl panics while another panic is already unwinding. What does the process do?
3. Suppose `in_flight` were a plain `usize` and the methods took `&mut self`. Could the guard hold `&mut Gateway` while `submit` keeps using `self`? What would you move onto the guard?
4. Two threads share one `Gateway`. What replaces the `Cell`, and which tokio type already is this guard?

## In the wild
- std [`MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html) unlocks when dropped. [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) closes when dropped.
- tokio [`SemaphorePermit`](https://docs.rs/tokio/latest/tokio/sync/struct.SemaphorePermit.html) gives its permit back when dropped: the same guard as this kata, across tasks.
- sqlx [`Transaction`](https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html) rolls back when it is dropped without `commit`.
- [`scopeguard`](https://crates.io/crates/scopeguard) (`defer!`, `guard`) for one-off cleanup that has no type of its own.
- Exchange-style: a per-session cap on open orders, or a risk-limit reservation, that is given back when the order is acked, rejected or the handler fails.

In a review, check:
- Paired `acquire`/`release` (or `lock`/`unlock`, `begin`/`end`) calls in a function with more than one exit.
- `let _ = guard_returning_call();`: the guard drops at once.
- `mem::forget` on a guard, or a `Drop` impl that can panic.
