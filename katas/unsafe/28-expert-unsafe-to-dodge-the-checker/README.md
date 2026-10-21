# 28 expert: unsafe to dodge the borrow checker

**Concept:** `unsafe` doesn't switch the borrow rules off; it switches the checking off. A reference into a Vec that survives `push` dangles when the Vec reallocates. The program may still print the right answer, which is why Miri exists.
**Source:** Rust for Rustaceans ch. 1, pointers point to places, https://readwise.io/open/1041420683; Miri, https://github.com/rust-lang/miri
**Fails with:** `Undefined Behavior`
**Vault:** your note on what unsafe actually allows

## Task
Make `cargo +nightly miri test` pass without changing the test. No `unsafe` left in `src/`.

This is an `expert` kata: the bug compiles and may even pass `cargo test`. Run `cargo +nightly miri test`; CI does too.

## Before you fix it
1. Run `cargo test`, then `cargo +nightly miri test`. Why does only Miri fail?
2. The test made capacity 1. What happens to the first element's address on the first `push`?

## Answer in the PR
1. What rule did `unsafe` let you break, and what did it cost?
2. Why did plain `cargo test` pass?
3. Name a case where `unsafe` is the right tool, and what it must promise.

## Think further
1. The test uses `Vec::with_capacity(1)`. With `with_capacity(10)`, no `push` reallocates. Would Miri still complain about the original code? Is the code then correct, or only lucky? (Read `Vec::as_ptr`'s note on the aliasing model.)
2. Make the element type `String` instead of `i32`. What stops `v.push(*first)` compiling even in your fixed version, and what does each way around it cost?
3. `for x in v.iter() { v.push(*x); }` is the same bug as a loop. Safe Rust rejects it; C++ calls it iterator invalidation. What would Miri report if `unsafe` forced it through?

## In the wild
- The Reference, [Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html): "`unsafe` only means that avoiding undefined behavior is on the programmer". Dangling places are on the list.
- `Vec`'s [Guarantees](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees) section: when `push` and `insert` may reallocate, which moves every element.
- The Book, [Creating a Safe Abstraction over Unsafe Code](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html): `split_at_mut` is `unsafe` used for a reason the checker can't see (two disjoint halves), not to silence a real conflict.
- [slab](https://docs.rs/slab/0.4.12/slab/): order books often keep orders in a slab and store keys (`usize`) in each price level, so adding an order never invalidates another order's handle.
- Clippy [`undocumented_unsafe_blocks`](https://rust-lang.github.io/rust-clippy/master/index.html#undocumented_unsafe_blocks) forces a `// SAFETY:` comment on every `unsafe` block.

In a review, check:
- An `unsafe` block added in the same PR as a borrow-checker error it "fixes".
- A reference or raw pointer into a `Vec`, `String` or `HashMap` that is still in use after a `&mut self` call on that container (`push`, `insert`, `reserve`, `extend`).
- `unsafe` with no `// SAFETY:` comment, or one that restates the code instead of naming the invariant.
- A crate with `unsafe` but no Miri run in CI.
