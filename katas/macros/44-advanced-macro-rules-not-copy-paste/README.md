# 44 advanced: `macro_rules!`, not copy-paste

**Concept:** The same trait impl pasted for five types drifts: a fix lands in four copies and misses the fifth. A declarative macro writes the impl once and stamps it out per type. `#[macro_export]` lets other crates call it, and `$crate::` paths make it expand correctly wherever it's called.
**Source:** Rust for Rustaceans (Jon Gjengset), ch. 7 Macros, "Declarative Macros" (when to use them, how they work, hygiene); The Rust Reference, Macros By Example, https://doc.rust-lang.org/reference/macros-by-example.html
**Fails with:** `E0433`
**Vault:** your note on declarative macros and hygiene

## Task
Make `cargo test` pass without changing the test. Keep `Amount`, `label`, `total` and the five public type names. In `src/`, write the `impl … Amount for` exactly once, inside one `#[macro_export] macro_rules! impl_amount`, and invoke it for the five types. The macro implements `Amount` for a one-field tuple struct the caller has already defined; it doesn't define the struct. No proc macros, no new crates, no `unwrap`, `expect` or `panic!`.

## Before you fix it
1. Run `cargo test`. `E0433` names a path. Which crate is it looking in, and what's missing there?
2. Diff the five impls by eye. Apart from the type name and the unit, which lines differ? Predict which test each difference breaks.
3. In `tests/kata.rs`, `mod outside` imports nothing. What happens if your macro writes `impl Amount for $t`?

## Answer in the PR
1. Which two copy-paste bugs were in the five impls, and which test would have caught each one if the code had compiled?
2. Which fragment specifiers did you choose for the type and for the unit (`ident`, `ty`, `literal`, `expr`, `tt`)? What would each of the others accept or reject?
3. Why does `$crate::Amount` work from `tests/kata.rs` when `Amount` and `crate::Amount` don't? What does `crate::` point at once the macro expands in another crate?
4. Could default methods on `Amount` have removed the copies instead? Which parts of the impl can a default method write, and which can't it (think `UNIT`, `new`, `minor`)?
5. What does the macro cost you in error messages, rust-analyzer, docs and reading the code? When would you keep hand-written impls?

## Think further
1. The caller has its own `Option` or a `mod core` in scope where they invoke your macro. Which names in your expansion would resolve to theirs, and how do `::core::…` paths avoid that?
2. A sixth type needs a different `checked_add`, for example a quantity capped at a maximum lot size. Would you add a macro arm, or hand-write that one impl? What does each do to "written once"?
3. Your macro calls `Self(minor)`. What error does a caller get if they pass a struct with a named field, and where does rustc point: at their call or inside your macro?
4. When does a declarative macro stop being enough, so that a `#[derive(Amount)]` proc macro is worth its extra crate?

## In the wild
- core's integer types: `uint_impl!` and `int_impl!` (defined in [`library/core/src/num/uint_macros.rs`](https://github.com/rust-lang/rust/blob/master/library/core/src/num/uint_macros.rs), invoked in [`num/mod.rs`](https://github.com/rust-lang/rust/blob/master/library/core/src/num/mod.rs)) write `checked_add` and its siblings once for every width.
- core's `impl_from!` in [`library/core/src/convert/num.rs`](https://github.com/rust-lang/rust/blob/master/library/core/src/convert/num.rs) stamps out the lossless `From` conversions between integer types.
- The Reference on [hygiene and `$crate`](https://doc.rust-lang.org/reference/macros-by-example.html#hygiene) and on [`#[macro_export]`](https://doc.rust-lang.org/reference/macros-by-example.html#path-based-scope).
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/): fragment specifiers, repetitions and follow-set rules.
- Exchange-style: unit newtypes (ticks, lots, quote cents, fee bps, sats) multiply with every new asset. One type that wraps or panics on overflow while the rest return `None` is a ledger bug, and a macro keeps the checked arithmetic identical everywhere.

In a review, check:
- Several impl blocks that differ only by the type name and a constant.
- An exported macro that names a crate item without `$crate::`, or relies on caller-shadowable names like `Option`.
- A macro with two or three call sites, where a generic function or a default method would read better.
