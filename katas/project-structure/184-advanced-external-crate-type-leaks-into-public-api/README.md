# 184 project-structure (advanced): another crate's type in your public API

**Concept:** Every type in a `pub` signature is part of your semver promise, so another crate's type there makes its major bump your breaking change and forces callers to depend on it. Wrap it in your own newtype, convert inside the crate, and keep the dependency's error as `source()`.
**Source:** Rust API Guidelines, "Necessities", C-STABLE ("Public dependencies of a stable crate are stable", https://rust-lang.github.io/api-guidelines/necessities.html); dtolnay's semver-trick README; katas 87 and 10. No Readwise highlight was available to the builder.
**Fails with:** `the public API names another crate's type`
**Vault:** your note on public dependencies and semver

`Price` and `PriceError` are this crate's public names, and `rust_decimal` does the parsing and arithmetic. The tests are downstream-style: they only use the public API and read `src/lib.rs` as text, so a leak shows up whether it is an alias, a re-export, an error variant or a trait impl.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the names `Price`, `PriceError` and its three variants, `parse_price` and `total`. No `unwrap`, `expect` or `panic!` in `src/`, no new crates, and don't change `tests/` or `Cargo.toml`.

The scan rule: no line starting with `pub ` or `impl`, and no line inside a `pub enum`, may contain `Decimal` or `rust_decimal`.

## Before you fix it
1. Run `cargo test`. Which three tests fail?
2. Write a downstream `match` on `PriceError::Invalid(e)` that logs `e`. What must its `Cargo.toml` contain?

## Answer in the PR
1. Which version bump of `rust_decimal` breaks your callers, and why does the alias make it your bump?
2. Why is a public `From<rust_decimal::Error>` impl part of your API?
3. What can a caller do with `source()` that they could not with the old Display, and what can they no longer do?
4. What does the newtype cost you, in methods you must now write by hand?

## Think further
1. When is `pub use rust_decimal;` the right call (for example, a crate that re-exports its core dependency), and what does it commit you to?
2. dtolnay's semver trick: make the old major version depend on the new one.
3. Cargo's unstable public/private dependencies (RFC 1977, `-Zpublic-dependency`) and the `exported_private_dependencies` lint. Check the status at build time.
4. Reading:
   - The Rust Book, "Packages, Crates, and Modules", and "More About Cargo and Crates.io". Check the current chapter titles on doc.rust-lang.org/book.
   - The Cargo Book, "Specifying Dependencies", "Features" and "SemVer Compatibility".
   - Rust API Guidelines, C-STABLE and C-NEWTYPE-HIDE.

## In the wild
- `reqwest::Error` keeps hyper's errors behind `source()` rather than exposing them.
- `std::io::Error` holds a boxed custom error behind `get_ref`.
- `http` is a deliberately public dependency of `hyper` and `axum`.

In a review, check:
- a `pub type` alias to a foreign type
- a foreign error inside a pub enum variant
- a public `From<ForeignError>` impl
- a foreign type in a `pub fn` signature

## Related concepts

### `pub use dep;` re-export
Makes a dependency part of your own module tree, so callers reach it through your crate. Use it when your API is built on that dependency and you want callers to use the same version.
```rust
pub use rust_decimal;
```

### Newtype for meaning
A one-field tuple struct that gives a value its own type, so the compiler keeps it apart from other values. Use it to make an API's types your own.
```rust
struct Meters(f64);
```

### `#[non_exhaustive]`
An attribute that stops callers from matching an enum or struct exhaustively, so you can add variants or fields later. Use it on public types you expect to grow.
```rust
#[non_exhaustive]
pub enum Kind { A, B }
```

### `Box<dyn Error + Send + Sync>`
A type-erased error that can hold any error and cross threads. Use it where a binary's `main` or a boxed source hides the concrete error type.
```rust
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}
```
