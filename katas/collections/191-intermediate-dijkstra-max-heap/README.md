# 191 intermediate: Dijkstra needs a min-heap

**Concept:** `BinaryHeap` is a max-heap; Dijkstra needs the smallest distance first, so wrap entries in `Reverse`. Skip stale entries (lazy deletion). Cost O((V + E) log V). Path weights are sums, so use checked arithmetic.
**Source:** std `BinaryHeap` docs, https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html; own design (shortest paths)
**Fails with:** `shortest_distances_on_fixed_graph`
**Vault:** your note on Dijkstra and the heap

## Task
Make `cargo test` pass without changing the tests. No new crates.

## Before you fix it
1. Which node pops first from a `BinaryHeap<(u64, usize)>`: the smallest distance or the largest?
2. What does `d + w` do in a debug build and in a release build when it overflows?

## Answer in the PR
1. Why is a stale heap entry harmless but a wrong pop order is not?
2. Where should overflow surface, and why not wrap?

## Think further
1. What breaks with negative weights, and which check in the code would catch them?
2. What would make the stale-entry skip unnecessary (a decrease-key operation)?
3. Why does `done[]` need the min-heap invariant to be correct?

## In the wild
- Routing and shortest-path services: road and network routing, where each edge weight is a travel time or a cost.
- `petgraph::algo::dijkstra`: the same algorithm over a graph type, with a priority queue of nodes.
- Schedulers that pick the earliest deadline or lowest cost next from a priority queue.

## Related concepts
- **`std::cmp::Reverse`:** a wrapper that flips the ordering of any `Ord` type. Use it to turn a max-heap into a min-heap or to sort descending.
```rust
let mut v = vec![3, 1, 2];
v.sort_by_key(|&x| std::cmp::Reverse(x));
```
- **`BTreeSet` as a priority queue:** an ordered set; `first()` gives the smallest item and `pop_first()` removes it. Use it when you need to remove an arbitrary entry, not just the top.
```rust
let mut pq = std::collections::BTreeSet::from([(5u64, 1usize), (2, 0)]);
let next = pq.pop_first();
```
- **`u64::checked_add`:** returns `None` on overflow instead of wrapping or panicking. Use it when a sum comes from untrusted input.
```rust
let total = 255u8.checked_add(1);
```
