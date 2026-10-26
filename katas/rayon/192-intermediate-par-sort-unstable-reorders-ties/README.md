# 192 intermediate: par_sort_unstable reorders equal keys

**Concept:** rayon has a stable parallel sort (`par_sort_by`) and an unstable one (`par_sort_unstable_by`). The unstable one may swap elements that compare equal. For price-time priority, equal prices must keep arrival order, so the name of the sort is a promise you must read.
**Source:** rayon `ParallelSliceMut` docs, https://docs.rs/rayon/1.12.0/rayon/slice/trait.ParallelSliceMut.html; std `slice::sort` vs `sort_unstable` docs (same stable/unstable split); own design (exchange order priority)
**Fails with:** `equal_prices_keep_arrival_order`
**Vault:** your note on stable vs unstable sorting

## Task
Make `cargo test` pass without changing the test. Keep the parallel sort. Do not sort sequentially to dodge the problem.

## Before you fix it
1. Run the test. Which orders changed places, and do they have the same price?
2. Read the sort's name in the docs. What does `unstable` say about equal elements?

## Answer in the PR
1. Why is order at a price an input the caller depends on, not an accident? Give the matching rule that needs it.
2. Does the stable parallel sort cost more than the unstable one? Say what the extra memory or work is, and when you would accept the unstable sort anyway.

## Related concepts
- **Stable sort (std):** `sort_by` keeps equal elements in order. Slower in general, but safe for multi-key sorts.
- **`sort_by_key` with a tuple key:** sorts by price then arrival id in one pass, so no tie is left for the sort to decide.

```rust
orders.sort_by_key(|o| (std::cmp::Reverse(o.price), o.id));
```
