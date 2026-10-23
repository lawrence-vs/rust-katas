//! 15 advanced: inject, don't reach for a global
//!
//! Think further, from the kata README:
//! 1. The rule is "hour < 12". At which exact second does the greeting change, and in whose time zone? Which part of the code should own the time zone: the clock, `greeting`, or the caller?
//! 2. A fixed clock answers "what time is it?". A test for "a session expires after 30 minutes" also needs time to move. What would a test clock need for that, and how does `tokio::time::pause` handle it for async code?
//! 3. `SystemTime` is not monotonic. If you measure how long a request took with two `SystemTime::now()` calls and NTP moves the clock in between, what do you get? Which clock should durations use, and does that one need injecting too?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::time::{Duration, Instant};

// Inject the clock so a test can pick the hour; the function never reads the system time itself.
trait Clock {
    fn hour(&self) -> u32;
}

struct FixedClock(u32);

impl Clock for FixedClock {
    fn hour(&self) -> u32 {
        self.0
    }
}

fn greeting(clock: &impl Clock) -> &'static str {
    if clock.hour() < 12 { "good morning" } else { "good afternoon" }
}

// Think further 2: a session expiry needs time to move, so the test clock advances on request.
struct TestClock {
    now: Duration,
}

impl TestClock {
    fn advance(&mut self, by: Duration) {
        self.now += by;
    }

    fn expired(&self, issued_at: Duration, ttl: Duration) -> bool {
        self.now.saturating_sub(issued_at) >= ttl
    }
}

fn main() {
    println!("{}", greeting(&FixedClock(9)));
    println!("{}", greeting(&FixedClock(15)));

    let mut clock = TestClock { now: Duration::ZERO };
    let issued = clock.now;
    clock.advance(Duration::from_secs(29 * 60));
    println!("at 29 min expired: {}", clock.expired(issued, Duration::from_secs(30 * 60)));
    clock.advance(Duration::from_secs(60));
    println!("at 30 min expired: {}", clock.expired(issued, Duration::from_secs(30 * 60)));

    // Think further 3: Instant is monotonic, so durations use it, not SystemTime.
    let started = Instant::now();
    let elapsed = started.elapsed();
    println!("measured with Instant: {}", elapsed.as_secs() < 1);
}
