# 05 intermediate: take, don't clone

**Concept:** Behind `&mut self` you may change a field but not move it out, because the struct must stay whole. `Option::take` moves the value out and leaves `None` behind: a move on purpose, not a clone.
**Source:** Network Programming With Rust (Chanda), mutable borrows, https://readwise.io/open/935627452
**Fails with:** `E0507`
**Vault:** your note on moving out of a borrow

## Task
Make `cargo test` pass without changing the test or the signature.

## Before you fix it
1. Why can't a value be moved out through `&mut`? What would `self` look like afterwards?
2. Try `.clone()` first and read which assertion fails.

## Answer in the PR
1. What does `take` leave behind, and why does that make the move legal?
2. What did `.clone()` cost here besides an allocation? (the test tells you)
3. Name the `std::mem` function that does the same for any `Default` type.

## Think further
1. Call `finish` twice on the same job. What does each call return, and is that the behaviour you want from a "collect the result" method?
2. What changes for the caller if `finish` consumed the job instead of borrowing it mutably? What can they no longer do afterwards?
3. What if `output` were a plain `String` instead of an `Option<String>`? What would "nothing left behind" have to mean?

## In the wild
- [`std::thread::JoinHandle::join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join): hands out the thread's result exactly once, by consuming the handle.
- [`tokio::sync::oneshot::Sender`](https://docs.rs/tokio/1.54.0/tokio/sync/oneshot/struct.Sender.html): `send` consumes the sender, so a value can't be delivered twice. Compare how you would get the same guarantee behind `&mut`.
- [`sqlx::Transaction`](https://docs.rs/sqlx/0.9.0/sqlx/struct.Transaction.html): `commit` and `rollback` take the transaction by value, so using it afterwards is a compile error rather than a runtime one.
- Exchange-style: a pending order slot that holds an outgoing message until the sender task collects it. Collecting must leave the slot empty, or the same order gets sent twice.

In a review, check:
- A `.clone()` on a field inside a `&mut self` method whose result is the field's old value.
- Whether "already taken" is a state callers can observe, and whether the type makes calling twice harmless.
- Methods that could consume `self` instead, so the type system enforces single use.
