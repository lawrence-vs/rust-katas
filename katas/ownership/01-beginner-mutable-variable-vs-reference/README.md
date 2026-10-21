# 01 beginner: mutable variable vs mutable reference

**Concept:** `let mut p = &x` lets you point `p` somewhere else; only `&mut x` lets you change `x` through `p`.
**Source:** Rust for Rustaceans, ch. 1 "Values, variables and pointers" (Listing 1-1).
**Vault:** your note on references (link it from its `code:` property).

## Task
Make `cargo test` pass. Don't change the test.

## Before you fix it
1. The error says "cannot assign to `*slot`, which is behind a `&` reference". Which of the two is read-only: the variable `slot` or what it points at?
2. Predict the one-word change.

## Answer in the PR
1. What is the difference between `let mut p = &x;` and `let p = &mut x;`?
2. The test passes `&mut x`. Would the old `&i32` parameter have accepted that? Why, given that nothing could be written through it?

## Think further
1. Once it passes, make `replace` generic over `T`. Which line stops compiling when `T` is `String`, and which rule from kata 02 does it break?
2. Write a call that passes the same `x` as both arguments. Does it compile? Why does the order in which arguments are evaluated matter?
3. Try to hold two exclusive references to `x` at once. What does the compiler say, and why does that rule make a function that writes through a reference safe to call?

## In the wild
- [`std::io::Read::read`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read): the caller's buffer is written through a parameter. The signature alone tells you the function writes into memory the caller owns.
- [`sqlx::Transaction`](https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html): queries run with `&mut *tx`. Every query borrows the transaction exclusively, so two queries can't interleave on it.
- [`tokio::sync::MutexGuard`](https://docs.rs/tokio/latest/tokio/sync/struct.MutexGuard.html): `*guard = v` writes through the guard. The lock is what grants the right to write, not the binding being `mut`.
- [`std::cell`](https://doc.rust-lang.org/std/cell/index.html): interior mutability, i.e. writes behind a shared reference. This is the legitimate exception to the rule, and the thing to question in a review.
- Exchange-style: an account balance or order-book level updated by a fill. The method that applies the fill should say in its signature that it mutates, so the borrow checker stops a reader from seeing a half-applied fill.

In a review, check:
- A function that writes says so in its signature. Be suspicious of `&self` methods that mutate through `Cell`/`RefCell`/atomics without a reason.
- `let mut` on a binding that is never reassigned (the compiler's `unused_mut` warning).
- Exclusive borrows held across `.await` or across a long block.
