/// The first letter of a display name, for an avatar.
/// CLUE: Does this function need to own the name, or only look at it? What does a caller holding a literal have to do to call it?
/// CLUE: `as_bytes()[0]` is one byte. How many bytes is `'É'`?
pub fn initial(name: String) -> Option<char> {
    if name.is_empty() {
        return None;
    }
    Some(name.as_bytes()[0] as char)
}

/// `name` cut to at most `max_chars` characters, for a fixed-width column.
/// CLUE: `len()` counts bytes and `max_chars` counts characters. Can those two be compared?
/// CLUE: A range on a `str` is in bytes. Which iterator hands you each character together with the byte where it starts?
pub fn truncate(name: String, max_chars: usize) -> String {
    if name.len() <= max_chars {
        return name;
    }
    name[..max_chars].to_string()
}
