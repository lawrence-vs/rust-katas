use kata_resilience_217::{Backoff, delay};
use proptest::prelude::*;
use std::time::Duration;

proptest! {
    #[test]
    fn delay_never_exceeds_max(attempt in 0u32..=200) {
        let base = Duration::from_millis(100);
        let max = Duration::from_secs(30);
        prop_assert!(delay(base, max, attempt) <= max, "delay above max at attempt {attempt}");
    }
}

#[test]
fn backoff_doubles_then_resets() {
    let base = Duration::from_millis(100);
    let mut b = Backoff::new(base, Duration::from_secs(30));
    assert_eq!(b.next_delay(), base);
    assert_eq!(b.next_delay(), base * 2);
    b.reset();
    assert_eq!(b.next_delay(), base);
}
