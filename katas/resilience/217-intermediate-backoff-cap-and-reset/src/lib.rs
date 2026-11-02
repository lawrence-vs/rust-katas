use std::time::Duration;

/// CLUE: does clamping the exponent clamp the delay?
/// CLUE: what is `base * 2^10` for a 100 ms base, next to a 30 s max?
pub fn delay(base: Duration, max: Duration, attempt: u32) -> Duration {
    let _ = max;
    base * 2u32.pow(attempt.min(10))
}

pub struct Backoff {
    base: Duration,
    max: Duration,
    attempt: u32,
}

impl Backoff {
    pub fn new(base: Duration, max: Duration) -> Self {
        Self {
            base,
            max,
            attempt: 0,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        let d = delay(self.base, self.max, self.attempt);
        self.attempt = self.attempt.saturating_add(1);
        d
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
    }
}
