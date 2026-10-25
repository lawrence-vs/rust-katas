# 163 advanced: a parser that never panics on garbage

**Concept:** A table test checks the inputs you thought of, a property test checks generated values against a rule, and a fuzzer feeds raw bytes nobody would type. For a parser of untrusted input the rule is "never panic: every input gives `Ok` or a typed `Err`".
**Source:** https://rust-fuzz.github.io/book/cargo-fuzz.html; kata 23; kata 161
**Fails with:** `parse_record panicked on hostile input`
**Vault:** your note on fuzzing and untrusted input

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`parse_record`, `Record`, `ParseError`, `MAX_NAME`). Don't change `tests/` or `Cargo.toml`. None of these in `src/`: `unwrap`, `expect`, `panic!`, `assert!`, `catch_unwind`, `#[allow(...)]`.

## Related concepts
### Table-driven tests
A list of inputs and expected outputs checked in one loop. Use it for the cases a reviewer should read, such as edges and known bugs.
```rust
for (input, want) in [("1", Some(1u8)), ("x", None)] {
    assert_eq!(input.parse::<u8>().ok(), want, "{input}");
}
```

### Property tests
Random inputs checked against a rule, with shrinking to the smallest failing input. Use it when the rule holds for every input, not just the ones you thought of (kata 161).
```rust
proptest! { #[test] fn doubles_are_even(n in 0u64..1_000) { prop_assert_eq!(n * 2 % 2, 0); } }
```

### cargo-mutants
Changes your code one small edit at a time and reruns the tests. A mutant that survives is a line no test checks. Use it to ask whether the tests are strong, not whether the code is right (kata 162).
```sh
cargo install cargo-mutants
cargo mutants   # MISSED = a change no test noticed
```

## Before you fix it
1. Run `cargo test`. Which input failed first, and which panic message printed above the assert?
2. For each arm of `hostile`, which line of `parse_record` does that garbage reach?
3. Predict the `ParseError` for `b"apple,12"` and for `b"\x00\xFF"`.

## Answer in the PR
1. List every panic site you found, the kind of garbage that reaches it, and the input the generator printed.
2. After your first fix, did the test fail on a different input? Why does a generator find one panic at a time?
3. Table tests, property tests, fuzzing: where does each get its inputs, and which needs you to imagine the bad input first?
4. Name a fix that satisfies "never panic" but fails `typed_result_for_each_kind_of_input`.
5. Why is a panic in a parser of network input a denial-of-service bug, not just a crash? (Kata 23.)

## Think further
None of this runs in CI: it needs nightly and libFuzzer, and a fuzz run has no natural end.
1. `cargo install cargo-fuzz`, then from this kata's directory run `cargo fuzz init`. It creates a `fuzz/` directory with a target file under `fuzz/fuzz_targets/`. Replace that file's contents with:
   ```rust
   #![no_main]
   #[macro_use]
   extern crate libfuzzer_sys;
   extern crate kata_testing_163;

   fuzz_target!(|data: &[u8]| {
       let _ = kata_testing_163::parse_record(data);
   });
   ```
   Run `cargo +nightly fuzz run <target>` on the starting code. How long until the first crash, and where is it saved (`fuzz/artifacts/<target>/`)?
2. Run the same command with the crash file as a second argument to reproduce it. Then turn the input into a table case.
3. libFuzzer is coverage-guided; `hostile` is blind. What can libFuzzer reach that `hostile` can't? Why seed `fuzz/corpus/<target>/` with the examples in `examples/think_further.rs`?
4. The target writes `let _ =`, which kata 124 calls a bug. Why is it right in a fuzz target?
5. How would you run it on a schedule anyway (`-- -max_total_time=300`), and why not on every push?

## In the wild
OSS-Fuzz runs fuzzers on open-source parsers continuously. The rust-fuzz trophy case (https://github.com/rust-fuzz/trophy-case) lists real crates whose bugs fuzzing found. File formats, wire protocols and config loaders all read bytes from strangers.

In a review, check:
- `unwrap` or `[i]` on anything derived from input
- `assert!` used for input validation
- a count check that is `>` where it should be `!=`
- `parse().unwrap()` on a field
