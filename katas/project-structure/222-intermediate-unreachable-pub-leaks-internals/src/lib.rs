#![deny(unreachable_pub)]

mod parse {
    /// CLUE: can any crate other than this one name `parse::tokenize`?
    pub fn tokenize(input: &str) -> Vec<&str> {
        input.split_whitespace().collect()
    }
}

pub fn word_count(input: &str) -> usize {
    parse::tokenize(input).len()
}
