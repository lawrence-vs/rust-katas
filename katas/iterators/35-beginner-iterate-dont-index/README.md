# 35 beginner: iterate, don't index

**Concept:** An index loop makes you write the bounds yourself, and every bound is a chance to be off by one. `iter()` visits each element exactly once, so `filter`, `map` and `sum` say what you want with no index to get wrong.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 15 Iterators: "map and filter" and "Simple Accumulation: count, sum, product", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch15.html
**Fails with:** `index out of bounds`
**Vault:** your note on iterators vs index loops

## Task
Make `cargo test` pass without changing the test. In `src/`: no indexing (`fills[i]`), no loop counter, no `while` or `for` loop. Write `bought_qty` as one iterator chain that ends in `.sum()`.

## Before you fix it
1. Run `cargo test`. Which index panicked, and how long was the slice?
2. Changing `<=` to `<` makes the tests pass. Why is that not the fix this kata asks for?
3. What does `fills.iter()` yield: `Fill` or `&Fill`?

## Answer in the PR
1. Why did the loop panic, in terms of the slice's length and its last valid index? If the loop had started at `i = 1` instead, which tests would break, and would they panic or quietly return a wrong number?
2. What does `filter` pass to its closure, and why does `map(|f| f.qty)` give you a `u64` and not a `&u64`?
3. How does `sum` know to produce a `u64` here?
4. The index loop bounds-checked every `fills[i]`. What does the iterator version check instead, and why can't it go past the end?

## Think further
1. The total no longer fits in a `u64`. What does `.sum()` do in a debug test build, and in `--release`? How would you write it with `checked_add` and `try_fold`? (See kata 42.)
2. You need the position too, e.g. "the index of the first sell over 100". Which adaptor gives you the index without a counter, and which consumer stops at the first match?
3. You need each fill and the one after it (the change between consecutive fills). Is that a reason to go back to `fills[i + 1]`, or do `windows(2)` or `zip` cover it?
4. `bought_qty` takes `&[Fill]`. What would `impl IntoIterator<Item = &Fill>` let callers pass that a slice can't, e.g. a `VecDeque` or a `BTreeMap`'s values?

## In the wild
- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter), [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) and [`sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum): lazy adaptors, and the consumer that drives them.
- [`std::slice::Iter`](https://doc.rust-lang.org/std/slice/struct.Iter.html): it walks a pointer towards the slice's end, so there is no index for you to get wrong.
- Clippy's [`needless_range_loop`](https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop) and [`explicit_counter_loop`](https://rust-lang.github.io/rust-clippy/master/index.html#explicit_counter_loop): lints that flag exactly this loop shape.
- Exchange-style: the resting quantity on one side of an order book up to a limit price is the same chain: `levels.iter().filter(|l| l.price <= limit).map(|l| l.qty).sum()`. Fees across a day's fills have the same shape.

In a review, check:
- `for i in 0..v.len()` or a `while i < len` counter where `i` is only used for `v[i]`.
- `len() - 1` on something that can be empty: it panics with an overflow in debug, and in release it wraps to a huge index.
- `.collect::<Vec<_>>()` in the middle of a chain only to iterate it again.
