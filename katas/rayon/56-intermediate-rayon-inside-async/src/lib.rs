use rayon::prelude::*;
use std::fmt;
use std::time::{Duration, Instant};

/// How long one level's pricing takes.
pub const PRICING_TIME: Duration = Duration::from_millis(300);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level {
    pub price: u64,
    pub size: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValuationError {
    /// The thread doing the work panicked or went away before it answered.
    WorkerLost,
}

impl fmt::Display for ValuationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValuationError::WorkerLost => write!(f, "the valuation worker was lost"),
        }
    }
}

impl std::error::Error for ValuationError {}

/// Stands in for CPU-heavy pricing (a risk model, a fee schedule): it keeps
/// this thread busy for PRICING_TIME. Don't change it.
pub fn level_value(level: &Level) -> u64 {
    let until = Instant::now() + PRICING_TIME;
    while Instant::now() < until {
        std::hint::spin_loop();
    }
    level.price * level.size
}

/// Values a depth snapshot for an async handler.
/// CLUE: A `#[tokio::test]` runtime has one worker thread. What is that thread doing while `par_iter` runs?
/// CLUE: `par_iter` hands the levels to rayon's threads. So why can't the caller go on?
/// CLUE: Whatever you send to another thread must own its data. Which value has to move in?
/// CLUE: That other thread can fail. Which `ValuationError` variant is for that?
pub async fn book_value(levels: Vec<Level>) -> Result<u64, ValuationError> {
    Ok(levels.par_iter().map(level_value).sum())
}
