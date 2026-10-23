# 63 advanced: a generic method behind `dyn`

**Concept:** A trait object calls methods through a vtable with one slot per method. A generic method has a copy per type, so it can't have a slot. Fence it off with `where Self: Sized`, or move it to an extension trait, so the rest of the trait stays dyn compatible (the older name is "object safe").
**Source:** Rust for Rustaceans (Gjengset), ch. 3 Designing Interfaces, "Object Safety"; ch. 2 Types, dynamic dispatch; Rust Reference, dyn compatibility. No Readwise highlight linked yet.
**Fails with:** `E0038` from the compiler, because `Venue` is used as `dyn` while a method has generic type parameters.
**Vault:** your note on static vs dynamic dispatch.

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep `best_asks` generic over `I: IntoIterator<Item = &str>`, and keep `cheapest` taking `&[Box<dyn Venue>]`. Delete no method. No `unwrap`, `expect` or `#[allow(...)]` in `src/`.

## Before you fix it
1. Read E0038. Which method does the note name, and for what reason?
2. `best_asks` is called with a `[&str; 2]` and with a `Map<…>`. How many functions does the compiler generate, and which one would a `dyn Venue` vtable point to?

## Answer in the PR
1. Why can't `dyn Venue` exist while `best_asks` is a generic method on it?
2. Which fix did you pick (`where Self: Sized` or an extension trait), and what does the other one cost? Cover calling it on `&dyn Venue`, overriding it per venue, and the import a caller needs.
3. With your fix, does `venues[0].best_asks(["BTC-USD"])` compile? Why?

## Think further
1. `fn best_asks(&self, pairs: &mut dyn Iterator<Item = &str>)` is dyn compatible without any split. What does it cost per call, and for the caller?
2. Add `fn boxed_clone(&self) -> Self` to `Venue`. Why does that break `dyn` too, and how do crates like `dyn-clone` get `Box<dyn Trait>: Clone`?
3. Adding a generic method without a default or `where Self: Sized` to a published trait breaks downstream `dyn` users. Is it a semver-major change?
4. `async fn` in a trait: is the trait dyn compatible, and what does `async-trait` do about it? (This links to the async katas.)

## In the wild
- The Rust Reference, dyn compatibility (https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).
- std `Iterator`: `map`, `filter`, `by_ref` and friends carry `where Self: Sized`, which is why `Box<dyn Iterator>` works.
- `tokio::io::AsyncReadExt` and `futures::StreamExt`: extension traits with blanket impls over `?Sized`.
- `dyn-clone`.
- `async-trait`.
- Exchange-style: a router loads `Vec<Box<dyn Venue>>` from config, and one helper someone adds to the trait later breaks every caller.

## In a review, check
- a generic method added to a trait used as `dyn`;
- a method returning `Self` on such a trait;
- an extension trait missing `?Sized` in its blanket impl.
