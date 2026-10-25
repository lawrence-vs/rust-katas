# 161 intermediate: a wallet invariant that table tests miss

**Concept:** Hand-picked cases test each operation alone; a property test over random sequences of operations finds the order that breaks `total == free + reserved`, and shrinking cuts it to the shortest sequence that still fails.
**Source:** the [proptest book](https://proptest-rs.github.io/proptest/) (strategies, shrinking, failure persistence); docs.rs `proptest` (`prop::collection::vec`, `prop_assert_eq!`, `ProptestConfig`, `RngSeed`); kata 36, kata 53
**Fails with:** `total != free + reserved`
**Vault:** your note on invariants and property tests

## Task
Make `cargo test` pass without changing `Cargo.toml` or the existing tests; you may add table cases. Keep the public API. None of the following in `src/`: `unwrap`, `expect`, `panic!`, `as`, `saturating_*`, `wrapping_*`, `#[allow(...)]`.

## Related concepts
### Table-driven tests
A list of inputs and expected outputs checked in one loop. Use it for the cases a reviewer should read, such as edges and known bugs.
```rust
for (input, want) in [("1", Some(1u8)), ("x", None)] {
    assert_eq!(input.parse::<u8>().ok(), want, "{input}");
}
```

### cargo-fuzz
Coverage-guided fuzzing with libFuzzer that feeds raw bytes to a target. Use it on parsers and decoders that must never panic on any input (kata 163).
```rust
fuzz_target!(|data: &[u8]| {
    let _ = my_crate::parse(data);
});
```

### cargo-mutants
Changes your code one small edit at a time and reruns the tests. A mutant that survives is a line no test checks. Use it to ask whether the tests are strong, not whether the code is right (kata 162).
```sh
cargo install cargo-mutants
cargo mutants   # MISSED = a change no test noticed
```

## Before you fix it
1. Run `cargo test`. Which test passes, and which one fails?
2. What `ops` did proptest shrink to? Run it again. Is it the same, and why?
3. Play the shrunk sequence one op at a time on paper. After which op does `total == free + reserved` break?

## Answer in the PR
1. Paste the shrunk sequence from the starting code, and the one you got after your first fix. Why are they so short when the generated sequences had up to 39 ops?
2. Every hand-picked case passed. What does each one avoid that the shrunk sequences don't?
3. Did you keep `total` as a field or derive it, and what does the other choice cost?
4. The property also checks `is_ok()` against the state before the op, and that a refused op changes nothing. Name a wrong fix the invariant check alone would let through.
5. You added a named table case for each shrunk sequence. What does it give a reader that the proptest doesn't?

## Think further
1. A seed is fixed here (`RngSeed::Fixed(161)`) and `failure_persistence` is `None`. What could a fixed seed keep hiding forever, and when would a committed `proptest-regressions/` file serve you better?
2. How would you state "money is conserved" for a `transfer` between two wallets as a property?
3. If `reserve` returned a hold id and `release(id)` freed exactly that hold, what new invariant joins `total == free + reserved`?
4. In a small clean-code review, would you add proptest, or table tests and a "property tests later" note? What does each give the reviewer?

## In the wild
- Card payments: an authorization holds funds, a capture spends them and an expiry releases them; available plus held must equal the ledger balance.
- An exchange holds balance for open orders; a cancel that releases twice credits money that never existed.
- [`proptest-state-machine`](https://docs.rs/proptest-state-machine) generates sequences of commands against a reference model, the grown-up form of this kata's `op()`.
- Hypothesis (Python) stateful testing and QuickCheck do the same for other languages.

In a review, check:
- A stored field that must equal a sum of other fields.
- `saturating_*` on money: it hides the error and invents or loses value.
- A check against one field followed by a subtraction from another.
- Tests that only ever call each operation once, on a fresh value.
