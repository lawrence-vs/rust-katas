# 223 intermediate: an #[expect] that stopped being true

**Concept:** `#[expect(lint, reason = "...")]` works like `#[allow]`, but the compiler warns when the lint no longer fires. A stale `#[expect]` is a false statement in the code; `#![deny(unfulfilled_lint_expectations)]` makes it a build error.
**Source:** Rust Reference, lint check attributes, https://doc.rust-lang.org/reference/attributes/diagnostics.html ; RFC 2383, lint reasons, https://rust-lang.github.io/rfcs/2383-lint-reasons.html ; Kraken (MIT) pairs each production `expect` with an `#[expect]` stating its invariant (see the comment on `[workspace.lints.clippy]` in its root Cargo.toml).
**Fails with:** `unfulfilled_lint_expectations`
**Vault:** your note on lint attributes (`allow`, `expect`, `deny`, and reasons)

## Task
Make `cargo test` pass without changing the test. Keep `key()` and its return value. Make the code tell the truth about dead code. Do not replace the attribute with `#[allow]` and do not remove the `#![deny(...)]`.

## Related concepts

### `#[allow(lint)]`
Silences a lint unconditionally, with no check that the lint still fires. Use it when a lint is a known false positive you cannot fix.
```rust
#[allow(clippy::too_many_arguments)]
fn build(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) {}
```

### `#[warn]` and `#[deny]`
Set a lint's level for an item or a whole crate. Use `deny` to make a class of mistakes fail the build.
```rust
#![deny(missing_docs)]
```

### `#[cfg(test)]`
Compiles an item only in test builds. Use it for helpers that only tests call.
```rust
#[cfg(test)]
fn fixture() -> u8 { 1 }
```

## Before you fix it
1. Read the error. Which attribute does it point at, and what does it say was expected?
2. Who calls `legacy_key`, and what does that mean for the `dead_code` lint?

## Answer in the PR
1. How does `#[expect]` differ from `#[allow]`?
2. Why does a stale `#[allow]` stay silent forever?
3. What does the `reason` string buy a reviewer?

## Think further
1. Change `key()` to stop calling `legacy_key` and keep your fix. What does the compiler do now, and which attribute would you write?
2. `#[expect]` on a whole module covers many items. If one of them stops triggering the lint but another still does, is the expectation fulfilled? What does that cost you in precision?

## In the wild
- [Rust Reference, lint check attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html): the `expect` attribute, the `reason` field and `unfulfilled_lint_expectations`.
- [RFC 2383, lint reasons](https://rust-lang.github.io/rfcs/2383-lint-reasons.html): why `expect` exists and how it differs from `allow`.

In a review, check:
- `#[allow]` where `#[expect]` would catch it going stale.
- `#[expect]` with no `reason`, or a reason that no longer matches the code.
