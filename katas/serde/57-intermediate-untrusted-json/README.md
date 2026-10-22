# 57 intermediate: untrusted JSON

**Concept:** JSON from a client is an attacker's input. Read it into a strict wire type that rejects unknown fields, then convert it with `try_from` into newtypes that can't hold a negative, zero or fractional amount, so a bad order is an error that names the field, never an `Order`.
**Source:** the serde [container attributes](https://serde.rs/container-attrs.html) page (`deny_unknown_fields`, `try_from`); the serde_json `from_str` docs; Rust API Guidelines C-VALIDATE; katas 10, 42 and 53
**Fails with:** `a negative quantity was accepted`
**Vault:** your note on parsing at the boundary

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API, except that `Order`'s, `Price`'s and `Quantity`'s fields become private. None of the following in `src/`: `f32`/`f64`, `as`, `unwrap`, `expect`, `panic!`, `#[allow(...)]`.

## Before you fix it
1. Run `cargo test`. Which tests pass on the starting code, and why does `missing field` already work?
2. What is `-5i64 as u64`?
3. Paste the `postOnly` JSON into a test: what does `post_only()` return?

## Answer in the PR
1. What did the starting code do with `"postOnly": true`, and what would that order do on a real book?
2. Why does `deny_unknown_fields` go on the wire struct and not on `Order`?
3. Why are the wire's `price` and `quantity` `i64` and not `u64`? Give the error each one produces for `-5`, and say which names the field.
4. Why is `try_from` better than deserialising `Order` and then calling a `validate()` method? What can a caller forget?
5. Where could an `Order` with a zero quantity come from now? Explain in terms of private fields.

## Think further
1. `#[serde(flatten)]` doesn't work with `deny_unknown_fields`. Why not?
2. Be strict on requests, but what about responses from an upstream venue that may add fields (forward compatibility)?
3. How would you parse `"19.99"` into cents without `f64`?
4. Where does a body size limit go relative to `from_str` (katas 21 and 23)?
5. How do you get a field path for an error in a nested struct (the `serde_path_to_error` crate)?
6. What length and charset limits does `client_order_id` need?

## In the wild
- An exchange order API returns an "unknown parameter" error instead of ignoring it.
- A post-only (maker-only) flag that gets dropped turns a maker into a taker and changes the fee.
- A FIX engine rejects an undefined tag instead of skipping it.
- [`std::num::NonZero`](https://doc.rust-lang.org/std/num/struct.NonZero.html) is a std type with a checked constructor; the serde [`try_from`](https://serde.rs/container-attrs.html#try_from) attribute runs one for you at the boundary.

In a review, check:
- `Deserialize` derived directly on a type with invariants.
- No `deny_unknown_fields` on request types.
- `f64` money.
- `as` on values that came off the wire.
