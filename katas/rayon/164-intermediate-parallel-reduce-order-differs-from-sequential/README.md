# 164 rayon (intermediate): a parallel reduce that disagrees with the loop

**Concept:** rayon cuts the input into pieces, folds each piece from the identity, then combines the results. The answer only matches the sequential loop if the identity really is an identity (`op(id, x) == x`) and the combine is associative. The split points change with the thread count, so a test must compare against a sequential oracle on every pool size.
**Source:** rayon docs for [`ParallelIterator`](https://docs.rs/rayon/1.12.0/rayon/iter/trait.ParallelIterator.html) (`fold`, `reduce`, `try_fold`, `try_reduce`, `sum`) and [`ThreadPoolBuilder::num_threads`](https://docs.rs/rayon/1.12.0/rayon/struct.ThreadPoolBuilder.html); katas 42 and 59.
**Fails with:** `parallel result equals the sequential oracle`
**Vault:** your note on associative combines in data parallelism

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`EntryId`, `Cents`, `Entry`, `line`, `closing_balance`, `statement`) and keep `par_iter` in both functions. None of these in `src/`: `.iter()`, `with_min_len`, `with_max_len`, `Mutex`, `sort`, `f32`, `f64`, `unwrap`, `expect`, `panic!`. No new crates.

## Before you fix it
1. Run `cargo test`. Which four tests fail, and which three pass?
2. `closing_balance_counts_the_opening_balance_once` runs on a pool with one thread, yet it gets 2150. How many pieces did rayon make, and why?
3. Predict how far off the 1-thread oracle result is, in cents.

## Answer in the PR
1. For each function, name the identity and the operation you used, and show that `op(identity, x) == x` holds.
2. Why must the operation be associative but need not be commutative? What does rayon promise about the order in which pieces are combined?
3. Why can checking each partial sum for overflow give `None` on one pool and `Some` on another? What did you do about it?
4. `fold` + `reduce`, `map().sum()`, or `collect` + `join`: which did you use where, and why?
5. Why does the test run the same input on 1, 2, 3, 4 and 8 threads instead of just one?

## Think further
1. `with_min_len(entries.len())` makes the tests pass. Why is that a hidden bug rather than a fix?
2. Write a float version of `closing_balance` and run it 100 times on 8 threads. Does the result change between runs? Why?
3. How would you compute the lowest running balance in parallel? Hint: combine `(sum, lowest_prefix)` pairs.
4. When is `sum::<i128>()` over `i64` amounts still at risk of overflow?

## In the wild
- End-of-day closing balances across many wallets.
- Building statements and exports from a ledger in parallel.
- Totals in reconciliation reports that must match a reference total to the cent.

In a review, check:
- a `fold` or `reduce` identity that is a real value (an opening balance, a fee, a header) instead of a neutral one
- a combine that adds a separator or a constant
- `f64` sums of money inside `par_iter`
- a parallel test that runs on only one pool size, or with no sequential oracle

## Related concepts
### `reduce_with`
A reduce with no identity that returns `None` for empty input. Use it when no value is a natural identity.
```rust
let max = v.par_iter().copied().reduce_with(i64::max);
```

### `try_reduce`
A reduce that stops at the first `None` or `Err`. Use it when the combine can fail.
```rust
let all: Option<u32> = v.par_iter().map(|&x| Some(x)).try_reduce(|| 0, |a, b| a.checked_add(b));
```

### `with_min_len`
Sets the smallest piece rayon will make. Use it to stop rayon splitting very cheap work too finely.
```rust
v.par_iter().with_min_len(1_024).map(work).collect::<Vec<_>>();
```

### Float addition
`f64` addition rounds after every step, so changing the grouping changes the sum. Use floats for measurements, not money (kata 42).
```rust
assert_ne!((0.1 + 0.2) + 0.3, 0.1 + (0.2 + 0.3));
```
