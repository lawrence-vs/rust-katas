# 74 intermediate: sparse ids, a map not a Vec

**Concept:** A `Vec` indexed by an id costs memory for every id up to the largest one you have seen, not for the ids you use. When ids are sparse, a map costs memory per entry instead, and `with_capacity` lets it do that up front, once.
**Source:** std `collections` docs, "When Should You Use Which Collection?", https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection; std `HashMap::with_capacity` docs
**Fails with:** `over the memory budget`
**Vault:** your note on choosing a collection

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API of `Ledger`. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Read the peak bytes in the failure message.
2. Compute `size_of::<Option<i64>>()`. Multiply by the largest id the test uses.
3. Five of the seven tests pass on the starting code. Why do they, and why do the other two not?

## Answer in the PR
1. Estimate bytes per entry for `Vec<Option<i64>>` and for `HashMap<u32, i64>`. For the map, count the 16-byte `(u32, i64)` tuple, the control byte, the 7/8 load factor and the power-of-two bucket count.
2. At what id density does the `Vec` win? What else does it give you (hashing, cache locality)?
3. Why does `with_capacity` take an account count, and what does `HashMap` promise about it?
4. Why does a `BTreeMap` fail the no-growth test?

## Think further
1. What does the starting code do with an id of `u32::MAX`? And what was the cost of `len`?
2. Ids are dense but start at a large offset, or you can map them once to `0..n`. What does a `Vec` plus an id-to-index map give you?
3. The default hasher is SipHash. For integer keys, `FxHashMap` or a no-op hasher is faster. What do you give up (HashDoS) and when is that fine?
4. After a burst of accounts is closed, what does `shrink_to_fit` do, and when is it worth it?
5. When does order matter enough to pay for a `BTreeMap`?

## In the wild
- [`HashMap::with_capacity`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.with_capacity): the capacity guarantee, and what it means for rehashing.
- Exchange account or instrument ids often start at large offsets, so they are sparse in `u32`. A log aggregator keyed by PID has the same shape.
- Clippy has no lint for this; it is a review check.

In a review, check:
- `resize(id + 1, ..)` where `id` comes from outside.
- `vec![x; MAX]` sized by the range of an id type.
- A `Vec` indexed by an external id.
