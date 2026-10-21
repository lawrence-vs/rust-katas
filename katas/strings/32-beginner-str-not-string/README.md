# 32 beginner: `&str`, not `String`

**Concept:** A function that only reads text should borrow it as `&str`: a literal, a `&String` and a slice of either all coerce to it, and the caller keeps what it owns. A `str` is UTF-8, so a character can be 1 to 4 bytes: `len()`, ranges and `as_bytes()` count bytes, and `chars()` and `char_indices()` count characters.
**Source:** Programming Rust, 2nd ed. (Blandy, Orendorff, Tindall), ch. 17 Strings and Text: "String and str" and "Iterating over Text", https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/ch17.html
**Fails with:** `E0308`
**Vault:** your note on `&str` vs `String` and UTF-8 bytes vs chars

## Task
Make `cargo test` pass without changing the test. In `src/`: both functions take `&str`; no `.to_string()`, `to_owned`, `clone`, `collect` or `String`. No `as_bytes`, and no `len()` compared with a count of characters. If you slice the name, the byte offset must come from `char_indices`. No `unsafe`, `unwrap` or `expect`.

## Before you fix it
1. Run `cargo test`. Which arguments does `E0308` point at, and what does the compiler suggest? Why can't you take that suggestion here?
2. How many bytes is `"Zürich"`, and how many characters? Check with `"ü".len()`.
3. Fix only the signatures first and run the tests again. Predict which tests fail, and how: a panic or a wrong value?

## Answer in the PR
1. Why can the test pass both `"Zürich"` and `&owned` to a `&str` parameter but neither to `String`? What would the compiler's `.to_string()` have cost at every call?
2. `truncate` returns `&str`. Whose bytes does it point at, which lifetime ties them together (you didn't write one), and why is there no allocation?
3. Why did `&name[..2]` panic on `"Zürich"`, while `as_bytes()[0] as char` quietly gave `'Ã'` for `"Émile"`?
4. Before your fix, why did `truncate("Zürich", 6)` drop the `h` without panicking?

## Think further
1. `"e\u{301}"` is an `e` followed by a combining accent: one letter on screen. How many `char`s is it, and what does your `truncate(.., 1)` return? What would you need to count what a user calls a letter?
2. A database column holds at most 32 **bytes**. Which `str` method tells you whether a byte offset is safe to cut at, and how would you find the largest safe offset at or below 32?
3. When is taking `String` the right signature? Think of a struct field, or a function that stores or returns the text. What does `Cow<'_, str>` give a function that only sometimes changes its input?
4. Where is `len()` the right answer? Think of buffer sizes and length prefixes on the wire.

## In the wild
- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars), [`char_indices`](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices) and [`is_char_boundary`](https://doc.rust-lang.org/std/primitive.str.html#method.is_char_boundary): the std tools for walking UTF-8 safely.
- [`String::truncate`](https://doc.rust-lang.org/std/string/struct.String.html#method.truncate) takes a **byte** length and panics off a char boundary. Read its "Panics" section.
- Clippy's [`needless_pass_by_value`](https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value) (pedantic) flags a `String` taken by value and only read, and [`ptr_arg`](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg) flags `&String` where `&str` fits.
- The [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) crate splits text into grapheme clusters, for Think further 1.
- Exchange-style: symbols and FIX tags are ASCII, so byte bugs hide in tests. Then a trader's display name, a venue label or a free-text order note arrives in UTF-8 and a fixed-width column or log line panics.

In a review, check:
- `fn f(s: String)` or `fn f(s: &String)` where the body only reads `s`.
- `s[..n]`, `s.as_bytes()[i]` or `s.len()` used where the code means characters.
- `.to_string()` at a call site only to satisfy a signature.
