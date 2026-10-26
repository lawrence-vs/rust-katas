# 193 intermediate: a slice that splits a UTF-8 character

**Concept:** `str` indexing takes byte offsets, and every index must sit on a character boundary. `é` is two bytes, so `&s[..1]` cannot take it whole. Count characters with `chars()`, or check the boundary with `is_char_boundary`.
**Source:** std `str` docs, indexing and `is_char_boundary`, https://doc.rust-lang.org/std/primitive.str.html#method.is_char_boundary; The Rust Book ch. 8, "Storing UTF-8 Encoded Text with Strings"
**Fails with:** `keeps_multibyte_char`
**Vault:** your note on bytes, chars and string slicing

## Task
Make `cargo test` pass without changing the test. Return the first `n` characters, not bytes. Do not allocate a new `String` unless you name why.

## Before you fix it
1. Read the panic message. Which byte index did it name, and why is that index inside a character?
2. Why does `takes_ascii_prefix` pass while the other test fails?

## Answer in the PR
1. What does `n` count in your fix, and why is that the right unit for a symbol name?
2. Your version has to walk the string. What does that cost compared to slicing, and when would you still slice?

## Related concepts
- **`char_indices`:** yields each character with its byte offset. Use it to find where the nth character starts.
- **`String` vs `&str`:** `&str` is a view, so a slice can't outlive its owner. Kata 32 covers the choice.

```rust
let ascii_len = name.len();
```
