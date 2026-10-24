# 104 intermediate: overflow checks differ by profile

**Concept:** Plain `+` on an integer panics on overflow in a debug build and silently wraps in a release build, so one input gives two answers; `checked_add` makes the answer an explicit error in every profile.
**Source:** Rust for Rustaceans (Gjengset), ch. 1, section "Cargo profiles" (idea only)
**Fails with:** `adding_past_u8_max_is_an_error`
**Vault:** your note on Cargo profiles and integer overflow

## Task
Make `cargo test` pass, and then `cargo test --release` too, without changing `tests/` or `Cargo.toml`. Keep the public API. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`.

## Before you fix it
1. Run `cargo test`. Read the panic message and which test it names.
2. Run `cargo test --release`. Which tests fail now, and is the message the same?
3. Four of the seven tests pass in both profiles on the starting code. Why do they?

## Answer in the PR
1. Why was it a bug? Say what `a + b` does for 200 and 100 in each profile, and why the caller cannot tell which answer it got.
2. Which fix did you pick, and what does the other option cost? Compare `checked_add` with `saturating_add` and `wrapping_add` for an amount that someone will be charged.
3. Why is the output of `cargo test` alone not enough evidence that this code is correct?

## Think further
1. What do `wrapping_add`, `saturating_add` and `overflowing_add` return for 200 + 100? When is each the right answer?
2. Setting `overflow-checks = true` under `[profile.release]` makes `+` panic everywhere. Why is that not the fix for input you do not control?
3. Which profile do `cargo test` and `cargo test --release` use, and what would a release-only bug need to hide?
4. Why is widening to `u16` before adding a different design from `checked_add`?

## In the wild
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#overflow-checks): `overflow-checks` is on in `dev` and off in `release` by default.
- A byte-sized counter in a log or ledger line quietly wraps to a small number in production while every debug run panics, or the other way round for a team that only ships debug builds.
- Clippy has `arithmetic_side_effects` (restriction, off by default) for this; otherwise it is a review check.

In a review, check:
- `+`, `-` or `*` on a value that came from outside the process.
- A test suite that only runs in the default profile.
- `as` casts or `wrapping_*` used to make an overflow "go away".
