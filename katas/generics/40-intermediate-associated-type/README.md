# 40 intermediate: an associated type, not a generic parameter

**Concept:** A trait can be generic in two ways: `trait Feed<T>` or `trait Feed { type Item; }`. A generic parameter lets one type implement the trait many times, once per `T`, so every bound, function and struct that uses it has to name `T`, even when it never chooses it; a struct that keeps no `T` needs `PhantomData<T>` just to hold it. An associated type allows one implementation per type: the implementor picks `Item`, and callers write `F: Feed` and `F::Item`, or pin it with `Feed<Item = Trade>`.
**Source:** Rust for Rustaceans (Gjengset), ch. 2 Types, "Traits and Trait Bounds": "Generic Traits"
**Fails with:** `E0107`
**Vault:** your note on associated types vs generic parameters

## Task
Make `cargo test` pass without changing the test. Neither `Feed` nor `Batcher` takes a type parameter for the event type (no defaulted one either), and there is no `PhantomData` in `src/`.

## Before you fix it
1. Run `cargo test`. Read each E0107: which items are "missing generics", and how many arguments does `Batcher` want?
2. List every place in `src/lib.rs` that names `T`. Which of them ever chooses what `T` is?

## Answer in the PR
1. With `Feed<T>`, how many `Feed` impls could `TradeFeed` have? With `type Item`? Which one matches what a feed is?
2. Why did `Batcher` need `PhantomData<T>`, and why can it go now?
3. State the rule of thumb for an associated type vs a generic parameter, and name one std trait of each kind.
4. What does `F: Feed<Item = Trade>` say that `F: Feed<Trade>` didn't, from the caller's side?

## Think further
1. Put back `impl Feed<Quote> for TradeFeed` next to the `Trade` impl on the starting code. What does `drain(&mut feed).len()` do now, and why does it compile with only one impl?
2. `Add<Rhs>` has a generic `Rhs` and an associated `Output`. Why does it want both? When would your `Feed` want a generic parameter as well?
3. A feed that lends out `&[u8]` frames borrowed from its own buffer can't express that with `type Item;`. What would `type Item<'a> where Self: 'a;` (a GAT) change?
4. `Feed` looks like `Iterator`. What would you gain by implementing `Iterator` instead, and what does `next_event` naming buy you?

## In the wild
- [`Iterator::Item`](https://doc.rust-lang.org/std/iter/trait.Iterator.html), [`Deref::Target`](https://doc.rust-lang.org/std/ops/trait.Deref.html) and [`FromStr::Err`](https://doc.rust-lang.org/std/str/trait.FromStr.html): one answer per type, so they're associated types. [`From<T>`](https://doc.rust-lang.org/std/convert/trait.From.html) and [`AsRef<T>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) are generic because one type converts from many.
- [`tower::Service<Request>`](https://docs.rs/tower/latest/tower/trait.Service.html) uses both: generic over the request (a service can accept many), with associated `Response`, `Error` and `Future`.
- Exchange-style: a feed handler is generic over `F: Feed<Item = MarketEvent>`; the decoder for each venue picks its event type once.

In a review, check:
- A trait parameter that every impl fills in the same way for a given type: make it an associated type.
- `PhantomData<T>` on a struct whose `T` only exists to satisfy a trait bound.
- Signatures with type parameters the caller never chooses (`fn f<T, F: Feed<T>>`).
