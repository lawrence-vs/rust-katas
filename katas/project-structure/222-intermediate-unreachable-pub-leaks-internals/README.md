# 222 intermediate: a pub item that nothing outside can reach

**Concept:** `pub` says "part of the public API", but a `pub` item inside a private module that nothing re-exports can never be reached from another crate. The `unreachable_pub` lint (allow by default) flags it so internals say `pub(crate)` or `pub(super)` instead.
**Source:** rustc lint `unreachable_pub`, https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unreachable-pub; the kraken-cli workspace turns it on as `unreachable_pub = "warn"` in `[workspace.lints.rust]` (MIT licensed, reimplemented here). No Readwise highlight.
**Fails with:** `unreachable_pub`
**Vault:** your note on visibility and the public surface.

## Task
Make `cargo test` pass by changing `src/lib.rs` only. Keep `word_count` public. Make the visibility of the internal item match what it really is. Don't re-export it and don't add `#![allow]`.

## Related concepts

### `pub(crate)`
Visible anywhere in the current crate, never outside. Main use: helpers shared between modules.
```rust
mod util {
    pub(crate) fn helper() {}
}
```

### `pub(super)`
Visible to the parent module only. Main use: a helper that one sibling-level caller needs.
```rust
mod a {
    pub(super) fn only_parent_sees_me() {}
}
```

### `pub use` re-export
Gives an item a public path outside its private module. Main use: a flat public API over private files.
```rust
mod inner {
    pub struct Thing;
}
pub use inner::Thing;
```

## Before you fix it
1. Run `cargo test`. Which attribute at the top of `src/lib.rs` turns the finding into an error?
2. Who, outside this crate, could call `parse::tokenize` today?

## Answer in the PR
1. Why does `pub` in a private module not make the item public?
2. What does `pub(crate)` allow that `pub(super)` does not?
3. Why is `unreachable_pub` a lint rather than an error by default?

## Think further
1. If `lib.rs` later adds `pub use parse::tokenize;`, what happens to the lint on that item, and why?
2. Rustc's `dead_code` and `unreachable_pub` both flag unused-looking items. How do their questions differ?

## In the wild
- [The Rust Reference: Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
- [rustc lint listing: unreachable-pub](https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unreachable-pub)
