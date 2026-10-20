/// Total number of bytes across all names.
/// CLUE: Does this function need to own the names, or only look at them?
/// CLUE: What does a caller have to do to keep using its Vec after calling this?
pub fn total_len(names: Vec<String>) -> usize {
    names.iter().map(|n| n.len()).sum()
}
