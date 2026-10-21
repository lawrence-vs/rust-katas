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
1. Make `until_char` return the text after the first `c` instead. Does anything about the borrows change?
2. Call `StrSplit::new` with a haystack read from a socket buffer and a delimiter that is a `const`. Which of the two lives longer, and does the caller ever see the difference?
3. Collect every item of a `StrSplit` into a `Vec<&str>`, then drop the delimiter. Should the `Vec` still be usable? Say why in terms of what each item points into.

## In the wild
- [`std::str::Split<'a, P>`](https://doc.rust-lang.org/std/str/struct.Split.html): which parameter carries the haystack's lifetime, and where does the pattern live? **Review check:** when a struct holds several references, does each one have a lifetime that matches its real source?
- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once): the signature takes a pattern and returns slices. Which input do the results borrow from? **Review check:** is the returned lifetime tied to only the argument the data comes from?
- [`std::str::pattern::Pattern`](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html): `&str`, `char` and closures all act as delimiters. What does that cost and buy a caller? **Review check:** does a public signature force callers to build a temporary only to satisfy a type?
- [`bytes::Bytes`](https://docs.rs/bytes/latest/bytes/struct.Bytes.html): slices of a network buffer without a lifetime parameter. What keeps the buffer alive? **Review check:** is a long-lived value holding a view into a buffer that is meant to be reused?
- [Exchange-style systems, serde: borrowing data](https://serde.rs/lifetimes.html): a parser borrowing field values from a message buffer and a separate lookup table of symbols. **Review check:** does the parsed message's lifetime follow the buffer only, or does it also get shortened by an unrelated input?
