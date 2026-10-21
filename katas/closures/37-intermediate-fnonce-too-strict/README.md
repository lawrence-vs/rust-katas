# 37 intermediate: fnonce-too-strict

**Concept:** A function that calls a closure multiple times but bounds it as `Fn` rejects callers that need `FnMut` (e.g., closures mutating captured state). The fix is the least restrictive bound the body actually needs: `FnMut` here.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 14 Closures, https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch14.html
**Fails with:** `E0596`
**Vault:** your note on closure trait bounds

## Task
Make `cargo test` pass without changing the test. No `.clone()` on the closure. The fix is a single bound change.

## Before you fix it
1. Run `cargo test`. The error says a value was moved. Which closure call moved it, and why does `Fn` not allow that?
2. The test passes a closure that mutates a local variable. Which trait does that closure implement, and which does it not?

## Answer in the PR
1. Why was it a bug, in terms of variables, values and pointers? What did the `Fn` bound promise that the body broke?
2. Which fix did you pick, and what does the other option cost? (The other option is changing the *body* to call the closure once. What would that prevent the function from doing?)
3. If the function only called the closure once, which bound would be the most permissive? Why is that bound still too restrictive for some callers?
4. In the trait hierarchy `Fn ⊂ FnMut ⊂ FnOnce`, which way does substitutability go? If a function asks for `FnOnce`, what can the caller pass? If it asks for `Fn`, what can the caller pass?

## Think further
1. A function takes `F: FnOnce(&i32)` and calls `f(x)` twice. What error do you get, and which rule does it name?
2. `for_each` on an iterator takes `FnMut`. Why not `Fn`? What would break if it took `Fn`?
3. A thread-spawning API takes `F: FnOnce() + Send`. Why `FnOnce` and not `FnMut`? What does `Send` have to do with the closure's captures?

## In the wild
- std [`Iterator::for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each) takes `FnMut`: it calls the closure once per element, and the closure may mutate state.
- std [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) takes `FnOnce() + Send`: the closure runs exactly once on another thread, so it must own everything it captures.
- tokio [`task::spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) same pattern: `FnOnce` future, `'static` + `Send`.
- Rayon [`ParallelIterator::for_each`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.for_each) takes `Fn`: it may call the closure concurrently, so the closure must not mutate captured state (or must use interior mutability).
- Exchange-style: a batch processor calls a callback per message. The callback tallies latency in a local `Vec`; it needs `FnMut`. Bounding it as `Fn` would reject this valid caller.

In a review, check:
- A generic function takes `F: Fn(...)`. Does the body call `f` more than once, or only read captured state? If it calls once, `FnOnce` is more permissive. If it calls many times but mutates, `FnMut` is the right bound.
- A callback parameter is `Fn` but the call site passes a closure that mutates a local. The fix is the function's signature, not the call site.
- `Fn` on a callback that is only invoked once (e.g., a one-shot handler). The caller can't pass an `FnOnce` closure that moves a value out.