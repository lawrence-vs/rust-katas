# 29 expert: a safe API over raw pointers

**Concept:** Raw pointers in a public API hand every invariant to every caller. Keep the pointer private, check bounds in safe methods, free it in `Drop`, and let `unsafe` shrink to a few audited lines (or disappear).
**Source:** The Rustonomicon, meet safe and unsafe, https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html
**Fails with:** `E0133`
**Vault:** your note on safe abstractions

## Task
Make `cargo test` and `cargo +nightly miri test` pass without changing the test. No public raw pointers, no `unsafe fn`, no leak (Miri reports leaks).

This is an `expert` kata: the bug compiles and may even pass `cargo test`. Run `cargo +nightly miri test`; CI does too.

## Before you fix it
1. Read E0133. Why can't the test call `get` without `unsafe`?
2. Who frees the allocation from `Box::into_raw`?

## Answer in the PR
1. Fix A: `Box<[u8]>` and no `unsafe`. Fix B: keep the pointer private with `Drop` and `// SAFETY:` comments. Which did you pick, and when is B worth it?
2. What did the leak look like in Miri's output?
3. Which invariant does each `// SAFETY:` comment rely on?

## Think further
1. `Buffer::new(0)`: what does the allocation look like, and what may `get(0)` do?
2. Someone adds `#[derive(Clone)]` to your fixed `Buffer`. With fix A, what happens? With fix B, what happens when both copies drop?
3. Is your `Buffer` `Send` and `Sync`? Why do raw pointers change the answer, and what would `unsafe impl Send` have to promise?
4. A hot loop wants to skip the bounds check. When is a `get_unchecked`-style `unsafe fn` worth it, and what would you measure first?

## In the wild
- The Nomicon, [Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html): std's `Vec` is this kata at full size. It has a private pointer, length and capacity, safe methods, and `unsafe` kept small inside.
- std slices offer both [`get`](https://doc.rust-lang.org/std/primitive.slice.html#method.get) and [`get_unchecked`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked). The unchecked one is `unsafe` because calling it with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
- [`bytes::Bytes`](https://docs.rs/bytes/1.12.1/bytes/struct.Bytes.html): the buffer under hyper, axum and tonic request bodies is a raw pointer, a length and a vtable, behind a fully safe API.
- Clippy [`missing_safety_doc`](https://rust-lang.github.io/rust-clippy/master/index.html#missing_safety_doc): a public `unsafe fn` needs a `# Safety` section that callers can actually check.

In a review, check:
- `pub` fields of raw pointer type, or a constructor that hands out a raw allocation.
- A public `unsafe fn` where a checked safe method returning `Option` or `bool` would do.
- A type that owns an allocation through a raw pointer and has no `Drop`, or derives `Clone`/`Copy`.
- A manual `unsafe impl Send` or `Sync` with no comment saying why it is sound.
