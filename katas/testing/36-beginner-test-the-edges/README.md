# 36 beginner: test the edges

**Concept:** A test that only tries the happy path proves little: the bugs live at the edges (no input, the largest input, rounding). A table of cases makes each edge one more row, and `#[should_panic(expected = "...")]` checks that a panic is the one you promised, not any panic.
**Source:** Rust for Rustaceans (Jon Gjengset), ch. 6 Testing, "Rust Testing Mechanisms" (the test harness, `#[should_panic]`)
**Fails with:** `attempt to add with overflow` (the edges test; the empty-input test fails with "panic did not contain expected string")
**Vault:** your note on testing the edges

## Task
Make `cargo test` pass without changing `tests/kata.rs`. In `src/lib.rs`, also rewrite `mod tests`: replace `mean_of_two` with one table-driven test (a slice of named cases, one `assert_eq!` in a loop) that covers at least one price, rounding down and an overflowing total, and give `empty_panics` an `expected` message. No `f64`, no `saturating_*`/`wrapping_*`, no `#[allow(...)]`.

## Before you fix it
1. Run `cargo test`. The unit tests in `src/` pass; which test in `tests/kata.rs` fails first, on which row, and with what message?
2. `empty_panics` passes on the starting code. Which line actually panicked, and with what message?
3. What is the largest value a `u64` can hold, and what is `u64::MAX + u64::MAX`?

## Answer in the PR
1. Why did `empty_panics` pass before your fix? What would `#[should_panic]` with no `expected` have let through if someone later broke the empty check?
2. Which type did you add the prices up in, and why can that total not overflow for any slice you can build in memory?
3. Why does `[u64::MAX, u64::MAX - 2]` catch an `f64` version that `[u64::MAX, u64::MAX]` would not?
4. Kata 07 says a library returns an error instead of panicking. Why is a documented panic acceptable here, and what would `Option<u64>` cost callers?

## Think further
1. You add a new edge (a slice of one `u64::MAX`). In a table test, what does it cost to add it, compared with a new `#[test]` function per case?
2. When one row fails, how does the `"case: {name}"` message help? What happens to the rows after it?
3. Property testing (e.g. `proptest`) generates the rows for you. What property would you state for `mean_price`? (Hint: compare it with the smallest and largest price.)
4. `cargo test --release` turns off overflow checks. What would the starting code have returned for the overflow row there, and would any test have noticed?

## In the wild
- The Rust Book, [`should_panic` with `expected`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic): the same check in the official tutorial.
- std documents panics in a `# Panics` section, e.g. [`slice::chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks) panics if the chunk size is 0, and [`Vec::remove`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove) panics when out of bounds.
- [`u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add) and widening to `u128`: the two usual ways to sum money-sized integers safely (see kata 42).
- Exchange-style: a volume-weighted average price over a day's fills is this function with weights. An empty window and a huge notional are the two cases that reach production untested.

In a review, check:
- `#[should_panic]` without `expected`.
- Tests that only try one "typical" input; no empty, one-element, maximum or rounding case.
- `iter().sum::<u64>()` over values that come from outside, with no wider type or `checked_*`.
