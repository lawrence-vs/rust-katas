# 13 beginner: a missing trait bound

**Concept:** A generic `T` can do nothing until a bound says what it can do. `>` is `PartialOrd::gt`, so comparing needs `T: PartialOrd`; floats are `PartialOrd` but not `Ord`.
**Source:** All Rust Features Explained, generics bound by traits, https://readwise.io/open/841530162
**Fails with:** `E0369`
**Vault:** your note on trait bounds

## Task
Make `cargo test` pass without changing the test.

## Before you fix it
1. Read E0369. Which trait provides `>`?
2. The test calls `largest` on `f64`. Does `f64` implement `Ord`? Why not?

## Answer in the PR
1. Why does a generic function need the bound, when every type in the test can be compared?
2. `PartialOrd` vs `Ord`: which did you pick, and what does NaN have to do with it?

## Think further
1. The slice holds a NaN: `[f64::NAN, 1.0]` and `[1.0, f64::NAN]`. What does `largest` return for each, and is "it depends on the order" acceptable for a function named `largest`?
2. Two items tie for largest. Does your `largest` return a reference to the first or the last one? Does `Iterator::max` agree, and when would the difference matter?
3. Prices at an exchange are often not floats (kata 42). If the price type were an integer newtype, which comparison traits would you derive on it, and what would that let you use that `f64` can't?

## In the wild
- [std `PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html): read the `partial_cmp` example with `f64::NAN` and the rule that `PartialOrd` must agree with `Ord`.
- [std `Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html): the total order that `sort`, `BTreeMap` keys and `Iterator::max` need.
- [`f64::total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp): std's total order over floats. Read where it puts NaN and `-0.0`.
- [`Iterator::max_by`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by): how std lets you pick the max of items that are not `Ord`.
- [ordered-float](https://docs.rs/ordered-float/latest/ordered_float/): a crate whose wrappers make floats `Ord`. Read what each wrapper does with NaN.
- Exchange-style: best bid and best ask are a max and a min over price levels. In a `BTreeMap` (kata 45) the key has to be `Ord`, and that is one more reason prices are not `f64`.

In a review, check:
- `partial_cmp(..).unwrap()` inside a `sort_by` or `max_by`: one NaN in the data and it panics.
- Floats used as sort keys, map keys or in `==`, with nothing said about NaN.
- A bound on the `struct` that only one `impl` needs. Bounds belong where they are used.
