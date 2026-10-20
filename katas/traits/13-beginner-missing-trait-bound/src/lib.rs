/// The largest item, or `None` for an empty slice.
/// CLUE: `>` is a trait method. Which trait, and does every `T` implement it?
/// CLUE: The test uses `f64`. Does `f64` implement `Ord`?
pub fn largest<T>(items: &[T]) -> Option<&T> {
    let mut max = items.first()?;
    for item in items {
        if item > max {
            max = item;
        }
    }
    Some(max)
}
