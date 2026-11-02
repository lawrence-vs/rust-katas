use std::time::Duration;

/// CLUE: what should a caller see when the condition never became true?
pub fn wait_until(mut cond: impl FnMut() -> bool, tries: u32) {
    for _ in 0..tries {
        if cond() {
            return;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}
