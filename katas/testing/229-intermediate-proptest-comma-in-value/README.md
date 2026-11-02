# 229 intermediate: a property test finds the comma

**Concept:** A property test finds the input a hand-written example never tries. A round-trip property (`decode(encode(x)) == x`) shows when a separator can appear inside a value.

**Source:**
Kraken uses proptest for round-trips, for example the tape ref Display/parse round-trip (`kraken-recording/src/tape.rs:344`) and the decimal round-trip (`kraken-core/src/request/decimal_wire.rs:131`) (MIT; reimplemented here, not copied).
- https://proptest-rs.github.io/proptest/
- https://docs.rs/proptest/latest/proptest/macro.proptest.html

**Fails with:** `round_trips`

**Vault:** Your note on property-based testing.

## Task
`encode` joins `key=value` pairs with `,`; `decode` splits them back. Make `cargo test` pass without changing the tests. Keep both signatures. The round trip must hold for every value, and the test strategy stays as it is.

## Related concepts

### `str::split_once`
Splits at the first occurrence of a separator. Use it to cut `key=value` where the value may hold more `=`.
```rust
assert_eq!("a=b=c".split_once('='), Some(("a", "b=c")));
```

### `char::escape_default`
Renders a char as an escape sequence. Use it to make a string safe for debug output.
```rust
assert_eq!("\n".escape_default().to_string(), "\\n");
```

### `prop_assert_eq!`
Proptest's assertion that returns an error instead of panicking, so the runner can shrink the input.
```rust
proptest! { #[test] fn t(x in 0u8..9) { prop_assert_eq!(x, x); } }
```

## Before you fix it
1. Run `cargo test`. What minimal input does proptest print?
2. Which character does `decode` read as a separator that a value may also hold?

## Answer in the PR
1. Why does a round-trip property beat a list of examples?
2. What is a minimal failing input, and why does proptest shrink it?
3. Why is restricting the strategy to avoid the bug a wrong fix?

## Think further
1. What if keys could hold `=` or `,` too?
2. When is length-prefixing a better format than escaping?

## In the wild
- [proptest book](https://proptest-rs.github.io/proptest/)
- [`proptest!` macro](https://docs.rs/proptest/latest/proptest/macro.proptest.html)
