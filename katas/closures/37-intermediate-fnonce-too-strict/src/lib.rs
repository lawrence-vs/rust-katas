/// Process each item with a callback.
/// CLUE: How many times does this function call the closure? What bound does that require?
/// CLUE: What happens if the caller passes a closure that mutates captured state?
pub fn process_each<F: Fn(&i32)>(items: &[i32], f: F) {
    for item in items {
        f(item);
    }
}
