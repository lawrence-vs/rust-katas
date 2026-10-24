# 68 intermediate: accidentally not Send

**Concept:** A future is `Send` only if everything it holds is `Send`. `Rc` is `!Send`, so a struct holding one turns a spawned future `!Send`, and `tokio::spawn` won't compile it. Use `Arc`, or don't keep the `!Send` value inside the future. Needs kata 16 (a std `MutexGuard` across `.await`) first: that is the same error from a different value.
**Source:** Rust for Rustaceans (Gjengset), ch. 2 Types and ch. 8 Asynchronous Programming
**Fails with:** `future cannot be sent between threads safely`
**Vault:** your note on Send, Sync and auto traits

## Task
Make `cargo test` pass without changing the test. Don't drop the shared `Venue`: don't clone the whole `Venue` into each task.

## Before you fix it
Which value does the error say is not `Send`, and is it noted as "captured" or "used across an await"?

## Answer in the PR
1. What is an auto trait, and why does `Rc` opt out of `Send`?
2. Why does moving the `format!` above the `.await` not fix it?
3. When is "drop the value before the await" enough, and why isn't it enough here?

## Think further
1. `Rc<RefCell<T>>` becomes `Arc<Mutex<T>>`: which half fixes `Send`, and which fixes `Sync`?
2. `tokio::task::LocalSet` and `spawn_local`: when is a `!Send` future fine?
3. How do you assert `Send` at compile time in a library's own tests?

## In the wild
- [std `Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html): scroll to "Auto Trait Implementations" and find `!Send`.
- [std `Send`](https://doc.rust-lang.org/std/marker/trait.Send.html): what an auto trait is and when a type gets it.
- [`tokio::spawn`](https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn.html): the `Send + 'static` bound is where this kata's error comes from.
- [Clippy `future_not_send`](https://rust-lang.github.io/rust-clippy/master/index.html#future_not_send) (nursery): a lint that flags a public async fn whose future is `!Send`.
- [tokio `LocalSet`](https://docs.rs/tokio/1.53.2/tokio/task/struct.LocalSet.html): the supported home for `!Send` futures.
- Exchange-style: read-only venue and fee config shared by every order task. Each task needs the fee table, none of them changes it.

In a review, check:
- `Rc`, `RefCell`, `Cell` or a raw pointer in a type that ends up in a spawned future.
- A `pub async fn` in a library with no test that asserts its future is `Send`.
