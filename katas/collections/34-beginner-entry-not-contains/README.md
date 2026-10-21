# 34 beginner: entry, not contains_key

**Concept:** Checking a map with `contains_key` (or `get_mut`) and then calling `insert` looks the key up two or three times, and a reference from the first lookup blocks the `insert`. `entry` does one lookup and gives you a handle to the slot, whether it was there or not.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 16 Collections: "Entries", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch16.html
**Fails with:** `E0499`
**Vault:** your note on the map entry API

## Task
Make `cargo test` pass without changing the test. In `src/`: no `contains_key`, `get_mut`, `insert`, `unwrap` or `expect`. Each function does one lookup per key, through `entry`.

## Before you fix it
1. Run `cargo test`. Which line holds the first mutable borrow of `book`, and which line wants the second one?
2. `count_fills` compiles. How many hash lookups does it do for a price it has already seen? And for a new one?
3. What type does `book.entry(price)` return, and what are its two variants?

## Answer in the PR
1. In `level_mut`, the `get_mut` on a miss returns `None`, so nothing is borrowed. Why does the compiler still think `book` is borrowed at `insert`?
2. What do `or_insert(0)` and `or_default()` each return, and why can you write `+= 1` or `.push(...)` straight onto it?
3. Why did your fixed `level_mut` compile when the starting one didn't? Talk about how many borrows of `book` there are and how long each lives.
4. `or_insert(Vec::new())` and `or_insert_with(Vec::new)`: when is the value built in each one, and when does that difference matter?

## Think further
1. The key is a `String` you only have as `&str`. `entry` takes the key by value. What does that cost on every hit, and how would you avoid it on a hot path?
2. A level should be removed once its last order is cancelled. What does `Entry::Occupied` let you do that `get_mut` doesn't?
3. Order books keep price levels sorted. Does `BTreeMap` have the same `entry` API? What changes about the cost of one lookup?
4. You want to count and also know whether this was the first fill at that price. Which `Entry` method or `match` gives you both from one lookup?

## In the wild
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) and [`std::collections::hash_map::Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html): `or_insert`, `or_insert_with`, `or_default`, `and_modify`.
- [`BTreeMap::entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.entry): the same API on a sorted map, which is what a price-level book uses.
- Clippy's [`map_entry`](https://rust-lang.github.io/rust-clippy/master/index.html#map_entry) lint flags `contains_key` followed by `insert`.
- The rejected `get_mut`-then-`insert` shape is "problem case #3" in the [NLL RFC](https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions); Polonius is meant to accept it one day, and `entry` is how you write it today.
- Exchange-style: "the level at this price, created on first order" and "fills per symbol" are both one `entry` call. A double lookup on the matching path costs a second hash for every order.

In a review, check:
- `contains_key` followed by `insert` or `get_mut(..).unwrap()` on the same key.
- `if let Some(x) = map.get_mut(k) { return x; } map.insert(..)`, or a workaround with a clone or a second map to get around it.
- `or_insert(expensive())` where `or_insert_with(expensive)` would skip the work on a hit.
