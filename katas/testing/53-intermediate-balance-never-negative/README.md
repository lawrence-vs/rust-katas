# 53 intermediate: a balance that never goes negative

**Concept:** A type whose constructor checks the rule (`TryFrom`) makes an invalid value impossible to hold, and a property test checks that no sequence of operations breaks the rule.
**Source:** the [proptest book](https://proptest-rs.github.io/proptest/); the std `TryFrom` and `i64::checked_sub` docs; Rust API Guidelines C-NEWTYPE and C-VALIDATE ("functions validate their arguments"); katas 10, 36 and 42
**Fails with:** `balance went negative` (the proptest; three example tests also fail)
**Vault:** your note on making invalid states unrepresentable

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API, except that `Balance`'s field becomes private. None of the following in `src/`: `unwrap`, `expect`, `panic!`, `as`, `wrapping_*`, `saturating_*`, `#[allow(...)]`.

## Before you fix it
1. Run `cargo test`. What `start` and `ops` did proptest shrink to?
2. What is `u64::MAX as i64`?
3. Which test passes on the starting code, and why?

## Answer in the PR
1. Where is the invariant enforced now, and why can no caller outside the module build `Balance(-5)`?
2. Which representation did you pick (`i64` or `u64`), and what does the other cost? Cover `TryFrom<i64>` and the maximum balance.
3. Why does the property compare against a model instead of only checking `>= 0`? Name a wrong fix that the sign check alone would let through.
4. What does shrinking give you that a random failing input wouldn't?
5. Why would `saturating_sub` be wrong for money?

## Think further
1. Should `proptest-regressions` files be committed in a real project? What does this kata's `failure_persistence: None` trade away?
2. `TryFrom<i64>` or `Balance::new(i64) -> Option<Balance>`: what does each give the caller?
3. How would a `Ledger` of many accounts state "money is conserved" as a property?
4. What would `cargo test --release` do to the starting `deposit`, and would any test notice?
5. How would serde's `#[serde(try_from = "i64")]` keep the invariant at the JSON boundary? (Kata 57 picks this up.)

## In the wild
- An exchange keeps an available balance and a held balance per account; an order moves cents from one to the other, and neither may go below zero.
- A broker rejects a withdrawal larger than the settled cash, even when unsettled trades would cover it.
- [`std::num::NonZero`](https://doc.rust-lang.org/std/num/struct.NonZero.html) is a std type with a checked constructor: the same idea for "not zero".
- `quickcheck` and `proptest` both do property testing; proptest's strategies and shrinking are explicit values you compose, as `op()` does here.

In a review, check:
- `pub` fields on a type with an invariant.
- `as` between signed and unsigned integers.
- `From` where the conversion can fail and needs `TryFrom`.
- Tests with a single example for a rule that should hold for every input.
