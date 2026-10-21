# 03 advanced: a struct outliving its reference

**Concept:** a reference is only valid while the variable it points at is alive. A lifetime on a struct is the compiler proving the struct can't outlive what it borrows.
**Source:** Rust for Rustaceans, ch. 1 (lifetimes are about places, not values).
**Vault:** your note on lifetimes.

## Task
Make `cargo test` pass. Two designs fix it; the question is which one you'd ship.

## Before you fix it
1. Read "`temp` does not live long enough". Where exactly is `temp`'s place destroyed?
2. Draw it: which variable holds the data, and which one holds the address?

## Answer in the PR
1. Fix A: move `temp` out of the inner block. Fix B: make `Connection` own a `String`. Which do you pick for a connection that lives for the whole program, and why?
2. When is a struct holding `&'a str` the better design? Name a real case.

## Think further
1. Change `reconnect` to return `&str` instead of `String`. What lifetime would the signature need, and which variable could the returned address borrow from?
2. Ten `Connection`s need the same URL, read once from a config file at startup. Who owns the URL text, and how long must that owner live?
3. Move a `Connection<'a>` into `std::thread::spawn`. What does the compiler ask of `'a`, and why does `std::thread::scope` ask for less?

## In the wild
- [`std::str::Split<'a, P>`](https://doc.rust-lang.org/std/str/struct.Split.html): what does `'a` tie it to, and how long does a `Split` usually live? **Review check:** a borrowing struct used inside one function is normal; one stored in a long-lived field is a question to ask.
- [`std::borrow::Cow<'a, B>`](https://doc.rust-lang.org/std/borrow/enum.Cow.html): one type with a borrowed and an owned variant. When would an API return it? **Review check:** does a lifetime parameter on a public type spread to every caller that stores it?
- [`tokio::spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) and [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html): compare their bounds. **Review check:** look for `Box::leak`, a stray `'static`, or a `.clone()` added only to get a value into a task (REVIEW.md 2.3/2.4).
- [`sqlx::query::Query<'q, DB, A>`](https://docs.rs/sqlx/latest/sqlx/query/struct.Query.html): what does `'q` borrow, and when is the query finished with it? **Review check:** a borrowed builder should not outlive the statement it executes.
- [Exchange-style systems, serde: borrowing data](https://serde.rs/lifetimes.html): an order message parsed from a socket buffer vs. an order resting in the book for hours. **Review check:** if a type borrows from a buffer, is that buffer reused or freed before the type is dropped?
