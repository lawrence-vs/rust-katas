/// Split one CSV line into its fields. A quoted field may contain commas.
/// CLUE: `split` cuts at every comma it finds. What must a splitter know about quotes that `split` doesn't?
pub fn split_fields(line: &str) -> Vec<String> {
    line.split(',')
        .map(|f| f.trim_matches('"').to_string())
        .collect()
}
