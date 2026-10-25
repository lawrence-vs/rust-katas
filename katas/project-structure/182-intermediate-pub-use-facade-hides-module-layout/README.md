# 182 project-structure (intermediate): a pub use facade hides the module layout

**Concept:** Callers that import `crate::internal::parser::tokens::Token` break whenever a module moves. A flat facade of `pub use` re-exports (renamed with `as` where internal names differ, plus a small `prelude`) gives every caller one stable path, while the tree behind it stays private and free to change.
**Source:** The Rust Book ch. 7 "Managing Growing Projects with Packages, Crates, and Modules", section "Bringing Paths into Scope with the `use` Keyword"; ch. 14 "More About Cargo and Crates.io", section "Publishing a Crate to Crates.io", subsection "Exporting a Convenient Public API"; the rustdoc book, "Re-exports"; kata 64. No Readwise highlight.
**Fails with:** `E0432`
**Vault:** your note on public API surface and re-exports.

This kata is one workspace with two packages. The root package (`src/`) has the module layout `internal/parser/{tokens,errors,lexer}.rs`. `refactored/` holds the same code in another layout, `syntax/{token,scan}.rs`, with different internal names. One test file, `tests/shared/public_api.rs`, is compiled into both packages, so each facade has to give the same public paths.

## Task
Make `cargo test` pass. Change only `src/` and `refactored/src/`. Don't change `tests/`, either `Cargo.toml`, the doc tests, `Token`, the error enums or the lexing logic. Don't move or rename files or modules. In `src/` and `refactored/src/`: no `unwrap`, `expect`, `panic!`, `#[allow]` or `#[doc(hidden)]`. The `/// CLUE:` lines sit on the `pub mod` lines in both `lib.rs` files.

## Before you fix it
1. Run `cargo test`. Which paths does `E0432` name, and in which file?
2. Write down the full path a caller needs to reach `Token` in each crate today.
3. Once the tests compile, which doc test fails, and why is a doc test the right place to check privacy?

## Answer in the PR
1. Before your fix, which change under `src/internal/` would have broken a caller's build? Which changes still would after it?
2. The two crates export the same paths from different trees. What did each facade say differently, and what did `as` buy you?
3. Why does `pub use …::tokens::*` fail a doc test here? What happens when two glob re-exports export the same name, and when does a user find out?
4. Why is `Error` not in your prelude?
5. Why does `#[doc(hidden)] pub mod internal` not count as private?

## Think further
1. rustdoc inlines items re-exported from a private module. When would you still write `#[doc(inline)]` or `#[doc(no_inline)]`? Read the rustdoc book's "Re-exports".
2. `#[doc(hidden)]` items are still public under semver. What do macro-support paths use it for?
3. A facade crate re-exports other crates. What happens to your semver when one of them has a major release?
4. How would `cargo-semver-checks` or `cargo public-api` catch a removed path?
5. Reading:
   - The Rust Book ch. 7, sections "Re-exporting Names with `pub use`", "Providing New Names with the `as` Keyword" and "Importing Items with the Glob Operator"
   - The Rust Book ch. 14, "Exporting a Convenient Public API"
   - The Cargo Book: "Workspaces", "Features" and "Specifying Dependencies"
   - The Rust API Guidelines, Documentation chapter

## In the wild
- [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html): defined in the public `hash_map` module and re-exported at `std::collections`.
- [`std::io::prelude`](https://doc.rust-lang.org/std/io/prelude/index.html): the `io` traits in one glob import.
- [`rayon::prelude`](https://docs.rs/rayon/latest/rayon/prelude/index.html): the parallel-iterator traits in one glob import.
- `serde` re-exports its `Serialize` and `Deserialize` derive macros behind its `derive` feature.

In a review, check:
- `pub mod` used where `mod` plus `pub use` would do;
- `pub use x::*` in a library root;
- callers importing three or more levels deep into a dependency.

## Related concepts

### `pub(crate)`
Visible across the whole crate, but not to other crates. Main use: a helper shared by many modules.
```rust
pub(crate) fn helper() {}
```

### `#[doc(hidden)]`
Keeps an item public and usable, but leaves it out of the generated docs. Main use: macro support paths that users must not call directly.
```rust
#[doc(hidden)]
pub fn __macro_support() {}
```

### The `#[path]` attribute
Loads a module from the file you name instead of the default location. Main use: a file layout that does not match the module name.
```rust
#[path = "v2/scan.rs"]
mod scan;
```

### Cargo workspaces
Several packages in one repository that share a lockfile and one output directory. Main use: a library and its helpers, built and tested together.
```toml
[workspace]
members = ["core", "cli"]
```
