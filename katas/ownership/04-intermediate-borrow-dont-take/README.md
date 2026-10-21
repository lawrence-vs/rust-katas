# 04 intermediate: borrow, don't take

**Concept:** A function that only reads data should borrow it. Taking `Vec<String>` by value forces every caller to give the Vec up, or `.clone()` it to keep using it.
**Source:** Network Programming With Rust (Chanda), introduction to the borrow checker, https://readwise.io/open/935627446
**Fails with:** `E0308`
**Vault:** your note on borrowing vs moving into a function

## Task
Make `cargo test` pass without changing the test. No `.clone()`, no `to_vec()`.

## Before you fix it
1. The test passes `&names`. What type does the function want, and what does the error say it got?
2. Which parameter type accepts a `&Vec<String>`, an array and a slice of a Vec?

## Answer in the PR
1. Why did the old signature force callers to clone, in terms of who owns the Vec?
2. `&Vec<String>` or `&[String]`: which did you pick, and what does the other cost a caller? (API Guidelines C-CALLER-CONTROL)
3. When is taking `Vec<String>` by value the right signature?

## Think further
1. After it passes, call the function with a `Vec<String>` you build inline, and then with a fixed-size array of `String`. Which calls compile, and what does each tell you about what the parameter accepts?
2. The caller holds the names behind an `Arc<Vec<String>>`, shared with another thread. What does it need to write at the call site, and does anything get copied?
3. Now the function also needs to sort the names for its own use. What are your options, and who pays for each?

## In the wild
- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains): a read-only query on text that every caller can invoke without giving up their `String`.
- [`sqlx::query`](https://docs.rs/sqlx/latest/sqlx/fn.query.html): the SQL text is passed as a borrow, so a `const` string and a built `String` both work and neither is consumed.
- [`http::HeaderMap::get`](https://docs.rs/http/latest/http/header/struct.HeaderMap.html#method.get): lookups hand back a view into the map, so the request stays usable afterwards.
- [`std::thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html): the opposite case. The closure must own what it captures, so the by-value signature is the honest one.
- Exchange-style: a risk check that totals exposure across a list of open orders. It runs on every incoming order, so taking the list by value would force a copy of the whole book per check.

In a review, check:
- Parameters of type `Vec<T>` or `String` on functions that only read them.
- A `.clone()` or `to_vec()` at a call site that exists only to satisfy a signature.
- Whether the parameter type is wider than needed, or narrower than the callers' data.
