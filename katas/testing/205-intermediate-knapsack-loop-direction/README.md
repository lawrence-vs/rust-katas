# 205 intermediate: knapsack loop direction

**Concept:** A one-row DP table stays 0/1 only if each item's pass reads the previous item's values, not values it already wrote in this pass. Table cases where the unbounded answer differs catch a loop that counts an item twice. Cost O(n x W) time, O(W) space.
**Source:** CLRS ch. 15 (dynamic programming) and Kleinberg and Tardos, *Algorithm Design*, ch. 6 (knapsack); own design (a cargo-loading problem); kata 36
**Fails with:** `case: one item counted once`
**Vault:** your note on dynamic programming state

## Task
Make `cargo test` pass without changing `Cargo.toml` or `tests/kata.rs`. You may add table cases in `tests/kata.rs` only if you keep the existing ones. Keep the public API (`Item`, `max_value`). No `unwrap`, `expect`, `panic!`, `as` or `#[allow(...)]` in `src/`.

## Related concepts
- **Unbounded knapsack:** each item may be reused any number of times. Use it when stock is unlimited, such as coin change.
```rust
let coins = [1usize, 5, 10];
// dp[a] = fewest coins that make amount a
let next = dp[a].min(dp[a - c] + 1);
```
- **Two-row (2D) DP table:** keeps the previous item's row explicit, so the read and the write cannot collide. Use it when you need to rebuild the chosen set.
```rust
let mut prev = vec![0u64; cap + 1];
let mut cur = vec![0u64; cap + 1];
```
- **Table-driven tests (kata 36):** loop over named cases, and put the case name in the assert message.
```rust
for (name, input, want) in &cases {
    assert_eq!(f(input), *want, "case: {name}");
}
```
- **Brute force over subsets:** tries every subset, so it is slow but obviously correct. Use it as an oracle in tests for a DP.
```rust
for mask in 0u32..1 << n {
    // keep the items whose bit is set in mask
}
```

## Before you fix it
1. Run `cargo test`. Which case fails first, and what does it print for `left` and `right`?
2. Define `best[w]` in one sentence, for the state after item `i`.
3. Trace `[(2,3)]` with capacity 4 by hand. Which entry of `best` is the answer, and how many copies of the item did it use?

## Answer in the PR
1. What is the state of `best[w]` after item `i`, and what is its base case?
2. Why does loop direction matter with one row? Which entries does `best[w - weight]` read at the moment it runs?
3. What are the time and space costs, in terms of items `n` and capacity `W`?
4. Which of the first three edge cases would a wrong fix still pass, and why keep them?
5. Name a case where the unbounded and 0/1 answers are equal. Why does a table without a case like that miss the bug?

## Think further
1. Weights up to 10^9: what breaks in `vec![0u64; capacity + 1]`, and what would you swap in?
2. How would you recover which items were chosen? What does the one-row table lose?
3. A property test against a brute-force subset oracle: what does it add over the table?
4. A zero-weight item: which loop direction still gives a correct answer, and why?

## In the wild
- Cargo and container loading: pick boxes under a weight limit, each box shipped once.
- Ad budget selection: pick campaigns under a spend cap, where each campaign runs at most once.
- Cutting-stock and capital budgeting problems with a single resource constraint.
- Review checks: in-place DP updates, loop direction against read-before-write, and tests where two different answers happen to coincide.
