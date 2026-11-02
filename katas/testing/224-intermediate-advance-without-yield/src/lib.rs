use std::time::Duration;

/// CLUE: `advance` moves the virtual clock; what makes a woken task actually run?
pub async fn advance_until(
    step: Duration,
    max_rounds: u32,
    mut done: impl FnMut() -> bool,
) -> bool {
    for _ in 0..max_rounds {
        if done() {
            return true;
        }
        tokio::time::advance(step).await;
    }
    false
}
