# 162 advanced: a mutant your tests let live

**Concept:** A mutant is your code with one small change, such as `>=` to `>`. A test that fails on it *kills* it; one that no test notices *survives*. Running every line (100% coverage) doesn't mean any test checked the comparison on that line.
**Source:** https://mutants.rs/; kata 36; kata 161
**Fails with:** `mutant survived: no case tells >= from >`
**Vault:** your note on mutation testing

## Task
Make `cargo test` pass by adding rows to `CASES` in `src/lib.rs`. Don't change `transfer_fee`, `transfer_fee_mutant`, the two tests, `tests/`, or `Cargo.toml`. No `unwrap`, `expect` or `panic!` in non-test code.

## Related concepts
### Line coverage
A count of which lines ran during the tests. Use it to find code no test reaches; it can't say whether a test checked a comparison on that line (`cargo llvm-cov`).
```sh
cargo llvm-cov --summary-only
```

### Property tests
Random inputs checked against a rule, with shrinking to the smallest failing input. Use it when the rule holds for every input, not just the ones you thought of (kata 161).
```rust
proptest! { #[test] fn doubles_are_even(n in 0u64..1_000) { prop_assert_eq!(n * 2 % 2, 0); } }
```

### Table-driven tests
A list of inputs and expected outputs checked in one loop. Use it for the cases a reviewer should read, such as edges and known bugs.
```rust
for (input, want) in [("1", Some(1u8)), ("x", None)] {
    assert_eq!(input.parse::<u8>().ok(), want, "{input}");
}
```

## Before you fix it
1. Run `cargo test`. Which test passes, and which one fails?
2. For each row in `CASES`, work out what `transfer_fee_mutant` returns.
3. Which rows would `amount > LARGE_TRANSFER` and `amount >= LARGE_TRANSFER` disagree on?

## Answer in the PR
1. What is a mutant, and what do "killed" and "survived" mean here?
2. Why does 99_999 not kill the mutant, and would 100_001?
3. The starting cases ran every line. Why did that prove so little?
4. Name one more mutant of `transfer_fee` that your table kills, and the row that kills it.

## Think further
1. `cargo install --locked cargo-mutants`. Its baseline needs green tests, so `cargo mutants --dir <this kata>` stops on the starting code. Run `cargo mutants --dir <this kata> -- -- --skip the_cases_kill_the_ge_mutant`, then read `mutants.out/missed.txt`. Does it list the `>` mutant from `src/lib.rs`? Why not?
2. `cargo mutants --list --diff --dir <this kata>`: which change does the tool make to `>=`, and which existing row already kills it?
3. When is a surviving mutant fine to leave (equivalent mutants, logging)? How would you mark it skipped?
4. Why run cargo-mutants on a schedule or on changed files, not on every CI push?

## In the wild
- Fee tiers, KYC limits, rate limits and age checks are all "at or above" rules. A one-off at the threshold is a classic production bug.

In a review, check:
- A `>`/`>=` with no test at the exact threshold.
- A coverage number cited as proof the tests are good.
- Tests that only pick values deep inside each branch.
