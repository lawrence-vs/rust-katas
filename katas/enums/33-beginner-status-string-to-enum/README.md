# 33 beginner: a status string to an enum

**Concept:** A status kept as a `&str` can be anything: a typo or a new value falls through an `if` chain to its `else` without a word from the compiler. Parse the string once, at the edge, into an enum; then `match` on the enum with every variant named, so a misspelt status doesn't compile and a new one is an error at every `match` that forgot it.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 10 Enums and Patterns: "Enums" (`http_status_from_u32`) and "Patterns" (exhaustive `match`), https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch10.html
**Fails with:** `E0432`
**Vault:** your note on enums and exhaustive matching

## Task
Make `cargo test` pass without changing the test. In `src/`: a status string is looked at only inside `parse_status`. `is_open` and `has_fills` are methods on the enum, each one `match` that names every variant: no `_` arm, no `matches!`, no `if` or `==`. No `unwrap`, `expect` or `panic!`.

## Before you fix it
1. Run `cargo test`. Which names does the test import that `src/lib.rs` doesn't have?
2. Work out `has_fills("partially_filled")` in your head. What does it return, and why didn't the compiler say anything?
3. `is_open("rejected")` and `is_open("rejectd")` both return `false`. Which branch decides each one?

## Answer in the PR
1. Where in your fixed code is a status still a string, and why should that be the only place?
2. Add an `Expired` variant locally and run `cargo build`. Quote the error code and the functions it points at. What would the string version have done with `"expired"`? (Remove the variant before you push.)
3. Why does `parse_status` need a `_` arm when `is_open` must not have one? What would `_ => false` in `is_open` cost you in question 2?
4. Which derives did the test make you add to `OrderStatus`, and which line of the test needs each one?

## Think further
1. Implement `FromStr` instead of `parse_status`. What does `"filled".parse::<OrderStatus>()` give you, and what error type would you pick? (Kata 09: `None` says something failed, not what.)
2. The status also goes back out on the wire. Write the signature of an `as_str` method. How would you test that every variant round-trips through `parse_status`?
3. An order is cancelled after a partial fill. What does a bare `Cancelled` lose, and what would `Cancelled { filled: u64 }` change in `has_fills`?
4. A library marks its enum `#[non_exhaustive]`. What must a `match` in another crate write, and what does that crate lose from question 2?

## In the wild
- [`std::cmp::Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html): the book's first enum; `match a.cmp(&b)` names `Less`, `Equal` and `Greater` instead of comparing integers to `-1`, `0` and `1`.
- [`std::io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html): a `#[non_exhaustive]` std enum, so code outside std must keep a `_` arm.
- serde's [`rename_all = "snake_case"`](https://serde.rs/container-attrs.html#rename_all) on a derived `Deserialize` enum is a ready-made `parse_status`: `"partially_filled"` becomes `PartiallyFilled` at the edge, and an unknown string is an error.
- Clippy's [`wildcard_enum_match_arm`](https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_enum_match_arm) (restriction, off by default) can ban `_` on enums in a crate that wants this rule enforced.
- Exchange-style: order and execution statuses arrive as strings in JSON or FIX messages. Parse them once in the client, and keep the matching engine and risk checks on the enum, so a new status the venue adds fails your build, not your book.

In a review, check:
- `status == "..."` or `.as_str() == ...` anywhere past the parsing function.
- `_ =>` on your own enum, where a new variant should be a compile error.
- An unknown string quietly mapped to a default variant instead of `None` or an error.
