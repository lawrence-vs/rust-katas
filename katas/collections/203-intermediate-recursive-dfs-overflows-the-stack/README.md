# 203 intermediate: a recursive DFS overflows the stack

**Concept:** A recursive search uses one stack frame per level, and a thread's stack is fixed (2 MiB by default for spawned threads), so a path as long as the input overflows it; an explicit `Vec` as the stack moves that depth to the heap at the same O(V + E).
**Source:** std `thread` docs, "Stack size", https://doc.rust-lang.org/std/thread/index.html#stack-size; kata 35
**Fails with:** `has overflowed its stack`
**Vault:** your note on recursion versus an explicit stack

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API and the visit order. No `stack_size`, no extra thread, no new crate. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Why does the whole test binary abort instead of one test failing?
2. Eight tests pass on the starting code. What makes the ninth different?
3. Predict the order an explicit stack gives on `preorder_follows_edge_insertion_order` if you mark nodes when you push them.

## Answer in the PR
1. How deep did the recursion get on the chain, and roughly how many bytes is one `visit` frame in a debug build?
2. Why did you mark visited on pop, and why push neighbours in reverse?
3. `Vec<bool>` versus `HashSet<usize>` for visited: memory per node, hashing, and when ids are not dense.
4. Show that the fix is O(V + E) in time and in extra memory. Count how many entries the stack can hold at once under mark-on-pop.
5. When is recursion fine (depth bounded by something you control, e.g. a balanced tree at log n), and when does input size make it unsafe?

## Think further
1. `reachable` walks the whole component. Stop as soon as you pop `to`.
2. A BFS with `VecDeque` gives a different order. What does it give you (shortest hop count)?
3. `Builder::stack_size(64 << 20)` passes the test. Why is that a patch and not a fix?
4. Release builds may inline or shrink frames. Why must you not rely on that?
5. Post-order (e.g. topological sort) on an explicit stack: what extra state does each entry need?

## In the wild
- `serde_json` has a default recursion limit of 128; `disable_recursion_limit` is paired with `serde_stacker` in its docs.
- rustc and `syn` hit deep-nesting overflows; the `stacker` crate grows the stack on demand.
- `petgraph::visit::Dfs` keeps an explicit stack plus a `FixedBitSet` for visited.
- Review checks: a recursive fn over an input-sized structure (graph, linked list, nested JSON), a `Drop` of a long `Box` linked list, and no depth limit on untrusted nesting.

## Related concepts
### `std::collections::VecDeque`
A ring buffer used as a FIFO queue. Use it for BFS and work queues.
```rust
let mut q = std::collections::VecDeque::new();
q.push_back(1);
assert_eq!(q.pop_front(), Some(1));
```

### `std::collections::HashSet`
A set of hashed keys. Use it for visited sets over sparse or non-integer ids.
```rust
let mut seen = std::collections::HashSet::new();
assert!(seen.insert(7));
assert!(!seen.insert(7));
```

### `std::thread::Builder::stack_size`
Sets a spawned thread's stack size. Use it when the recursion depth is bounded and known.
```rust
std::thread::Builder::new().stack_size(8 << 20).spawn(|| {})?;
```
