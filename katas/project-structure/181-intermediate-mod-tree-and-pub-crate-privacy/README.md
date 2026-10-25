# 181 intermediate: a mod tree where everything is pub

**Concept:** Privacy belongs to modules, not types: a private field or item is visible in its own module and that module's children. Keep fields private behind a checking constructor, and give each helper the narrowest visibility that still compiles: `pub(super)` for the parent only, `pub(crate)` for the whole crate, `pub` only for the public API.
**Source:** The Rust Book ch. 7 "Packages, Crates, and Modules", section 7.2 "Control Scope and Privacy with Modules"; The Rust Reference, "Visibility and privacy"; rustc lint `unreachable_pub`; Rust API Guidelines C-STRUCT-PRIVATE. No Readwise highlight.
**Fails with:** `Test compiled successfully, but it's marked` (all five doctests; the six tests in `tests/kata.rs` pass)
**Vault:** your note on module privacy.

## Task
Make `cargo test` pass by changing only visibility in `src/`. Don't change `tests/`, `Cargo.toml` or the five doc tests. Don't rename, add or remove any item or field, don't change any public path or signature, and don't use `#[allow]`, `#[doc(hidden)]`, `unwrap`, `expect` or `panic!`. `Percent`, `PercentError` and `Cart` with their methods are the only public API. Everything else gets the narrowest visibility that compiles.

## Before you fix it
1. Run `cargo test`. Which tests pass and which fail, and why do the six in `tests/kata.rs` pass?
2. A doc test is its own crate. Which of the five blocks would still compile if pasted inside `src/cart.rs`?
3. Change only `pub mod money;` to `mod money;` and run `cargo test`. What error do you get, and which line of `Cargo.toml` makes it an error?

## Answer in the PR
1. Why was it a bug? Build a `Percent` a caller could make before, and say what `Cart::total` returned for it with one line of 1000 cents.
2. For each item you narrowed, which visibility did you pick and which modules can see it now? Why `pub(super)` for `line_total` but `pub(crate)` for `share`?
3. `total.rs` reads `Line`'s fields with no `pub` on them. Why does that compile?
4. A `compile_fail` block passes on any compile error. Which wrong fix would pass all five, and how would you catch it in review?
5. Where does `mod total;` in `src/cart.rs` look for its file, and where would it look in the older `mod.rs` layout?

## Think further
1. `pub(in crate::cart)`: when is it clearer than `pub(super)`?
2. `use self::`, `super::` and `crate::`: which of them still work after you move a file, and which don't?
3. Reading:
   - The Rust Book ch. 7 "Packages, Crates, and Modules"
   - The Cargo Book: the pages on features, on specifying dependencies and on the `[lints]` table
   - Rust API Guidelines C-STRUCT-PRIVATE and C-VALIDATE

## In the wild
- [`std::string::String`](https://doc.rust-lang.org/std/string/struct.String.html): its `Vec<u8>` is private, so only valid UTF-8 gets in.
- [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html): private fields, and the nanoseconds part is always under 1e9.
- [`std::num::NonZero::new`](https://doc.rust-lang.org/std/num/struct.NonZero.html#method.new): returns an `Option`, so the invariant is checked once at the boundary.

In a review, check:
- a `pub` field on a type with an invariant;
- a `pub mod` that nobody outside the crate uses;
- `pub` inside a private module.

## Related concepts

### `pub use` re-export
A `pub use` gives an item a stable public path that can differ from its file layout. Main use: a clean public API over private modules.
```rust
mod internal {
    pub struct Thing;
}
pub use internal::Thing;
```

### `#[non_exhaustive]` (kata 64)
Stops other crates from building the type with a literal or matching it exhaustively. Main use: adding a field later without a breaking change.
```rust
#[non_exhaustive]
pub struct Config {
    pub timeout_secs: u64,
}
```

### A sealed trait (kata 64)
A public trait with a private supertrait, so other crates can call it but not implement it. Main use: controlling who may implement a trait.
```rust
mod private {
    pub trait Sealed {}
}
pub trait Shape: private::Sealed {}
```

### A builder
A separate type that collects settings one step at a time and checks them in `build()`. Main use: optional settings with validation.
```rust
let cart = CartBuilder::new()
    .discount(Percent::new(10)?)
    .build()?;
```
