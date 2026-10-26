/// The field at `index` in a comma-separated row, or `fallback` when the row is short.
/// CLUE: Elision ties an output to the one input reference. Here two references go in and one comes out. Which input does the result borrow from?
pub fn field_or(row: &str, index: usize, fallback: &str) -> &str {
    row.split(',').nth(index).unwrap_or(fallback)
}
