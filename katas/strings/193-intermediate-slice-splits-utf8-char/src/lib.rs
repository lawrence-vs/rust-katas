/// The first `n` characters of a symbol name, which may contain non-ASCII letters.
/// CLUE: `&s[..n]` counts bytes, not characters. What does a byte index that lands inside a character do?
pub fn first_chars(name: &str, n: usize) -> &str {
    &name[..n]
}
