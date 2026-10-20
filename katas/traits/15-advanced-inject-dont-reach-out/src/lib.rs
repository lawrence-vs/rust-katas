use std::time::{SystemTime, UNIX_EPOCH};

/// A greeting for the current UTC hour.
/// CLUE: How would a test make this return "good afternoon" at 9 in the morning?
/// CLUE: What if the caller handed in the thing that knows the time?
pub fn greeting() -> &'static str {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hour = (secs / 3600) % 24;
    if hour < 12 {
        "good morning"
    } else {
        "good afternoon"
    }
}
