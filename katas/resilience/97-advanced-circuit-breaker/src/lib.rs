use std::future::Future;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

/// Where the breaker reads the time from. Tests inject a fake one.
pub trait Clock {
    fn now(&self) -> Instant;
}

/// The real clock, for callers outside tests.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub failure_threshold: NonZeroU32,
    pub cool_down: Duration,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CallError<E> {
    #[error("circuit open: failing fast")]
    Open,
    #[error("dependency failed: {0}")]
    Inner(E),
}

/// CLUE: What must the breaker remember between calls: a count, a moment, or both?
pub struct Breaker<C> {
    #[allow(dead_code)]
    clock: C,
    #[allow(dead_code)]
    config: Config,
}

impl<C: Clock> Breaker<C> {
    pub fn new(clock: C, config: Config) -> Self {
        Breaker { clock, config }
    }

    /// CLUE: Which answer could the breaker give without calling the dependency at all?
    pub async fn call<T, E, F, Fut>(&mut self, mut op: F) -> Result<T, CallError<E>>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        /// CLUE: Three attempts per request, during an outage of a minute, with 1000 requests: what does that add up to?
        const ATTEMPTS: u32 = 3;
        let mut attempt = 1;
        let mut result = op().await;
        while result.is_err() && attempt < ATTEMPTS {
            result = op().await;
            attempt += 1;
        }
        result.map_err(CallError::Inner)
    }

    /// CLUE: The cool-down ends by itself. How can the breaker know that
    /// CLUE: without a timer running in the background?
    pub fn state(&self) -> State {
        State::Closed
    }
}
