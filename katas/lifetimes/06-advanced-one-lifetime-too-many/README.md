# 06 advanced: one lifetime too many

**Concept:** One lifetime on a struct with two references says both borrow from the same place. When the haystack and the delimiter come from different places, that single `'a` ties the output to the shorter-lived input. Give each source its own lifetime.
**Source:** Network Programming With Rust (Chanda), lifetime parameters on structs, https://readwise.io/open/935627468; Rust for Rustaceans ch. 1 as background
**Fails with:** `E0515`
**Vault:** your note on what a lifetime parameter claims

## Task
Make `cargo test` pass without changing the test or `until_char`'s signature. No `Box::leak`, no `'static`. Two designs work; which would you ship in a library?

## Before you fix it
1. Read E0515. The data `delim` owns dies at the `}`. Why does the compiler think the result points into it?
2. Draw it: which variable does each `&str` field point into?

## Answer in the PR
1. Fix A: a second lifetime for the delimiter. Fix B: a generic delimiter type (a `Delimiter` trait for `&str` and `char`), so `until_char` passes `c` directly. Which do you ship, and what does each cost a caller?
2. With one lifetime, the compiler picked the shorter of the two borrows. Why is that the only sound choice?

## Think further
1. `until_char`'s signature has no lifetimes written. Use the [elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html) to write them out. Which input does the output borrow from?
2. The haystack lives for the whole program, but each delimiter is a `String` built inside a loop. After your fix, which split items can you keep once the loop ends?
3. Edge cases: what does `next` yield for `"a b "` split on `" "`, and for `"abc"` split on `""`? Run it, then decide what the type should promise.

## In the wild
- [serde: understanding deserializer lifetimes](https://serde.rs/lifetimes.html): `'de` names the input buffer by itself, so a struct can borrow `&'de str` from the JSON it was parsed from.
- [`sqlx::Executor`](https://docs.rs/sqlx/0.9.0/sqlx/trait.Executor.html): methods such as `fetch` take `'e`, `'q` and `'c`, with the bounds `'q: 'e` and `'c: 'e`. Name what each lifetime is tied to.
- [The Book, ch. 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html): `longest`, where one lifetime on both inputs is the right claim. Compare it with `StrSplit`.
- Exchange-style: a matcher takes the order book and an incoming order and returns fills that point at resting orders in the book. With one lifetime, the fills are also tied to the incoming order, so they can't be kept once that order is dropped.

In a review, check:
- For every lifetime parameter, which input does it name? Watch for two references sharing one `'a` when they come from different places.
- Lifetimes that do nothing (clippy's `needless_lifetimes` and `extra_unused_lifetimes`, see the [lint list](https://clippy.rust-lang.org/master/index.html)).
- A `.to_string()` or `'static` at a call site that exists only to satisfy one shared lifetime.
