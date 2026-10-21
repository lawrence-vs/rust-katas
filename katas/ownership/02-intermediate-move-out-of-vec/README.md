# 02 intermediate: moving out of a Vec

**Concept:** `=` moves the value out of a place. A `Vec` owns its elements, so you can't move one out through an index: it would leave a hole.
**Source:** Rust for Rustaceans, ch. 1 (the `=` operator stores a value in a place).
**Vault:** your note on moves.

## Task
Make `cargo test` pass. CI also runs `cargo clippy --all-targets -- -D warnings`, so run that too. There are two good fixes; pick one, then explain the other.

## Before you fix it
1. Read "cannot move out of type `[String]`, a non-copy slice". Why would `&[i32]` compile?
2. Which trait makes the difference?

## Answer in the PR
1. Fix A: `.clone()`. Fix B: return `&str` borrowed from the vector. What does each cost, and who owns what afterwards?
2. With fix B, why don't you need to write a lifetime in the signature? (Look up lifetime elision.)

## Think further
1. What does the current indexing do on an empty inventory, and what return type would make that case explicit to the caller?
2. The caller wants to drop `inventory` straight after calling `equip`. Which of the two fixes still compiles, and why?
3. Suppose `equip` really should *take* the item, so the inventory shrinks. What would the signature need, and how do `Vec::remove` and `Vec::swap_remove` differ in cost and order?

## In the wild
- [`Vec::swap_remove`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove): the std way to move a value out of a place while filling the hole.
- [`tonic::Request`](https://docs.rs/tonic/0.14.6/tonic/struct.Request.html): `get_ref` borrows the message, `into_inner` consumes the request to move it out. It is the same choice the kata asks about.
- [`http::HeaderMap::get`](https://docs.rs/http/1.5.0/http/header/struct.HeaderMap.html#method.get) (used by axum): returns a reference into the map instead of an owned copy.
- [`VecDeque::pop_front`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front) vs [`front`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.front): owned vs borrowed access to the head of a queue.
- Exchange-style: a FIFO queue of orders at one price level. Matching pops the head order (moves it out), while a market-data snapshot only peeks (borrows). Mixing the two up either clones every order on every tick or fights the borrow checker during matching.

In a review, check:
- A getter that returns an owned `String` or struct where a borrow would do, and `.clone()` inside hot loops (see clippy's [lint list](https://clippy.rust-lang.org/master/index.html) for `redundant_clone`).
- Indexing with `[0]` on input that might be empty.
- Borrows returned from a collection that the caller then tries to mutate.
