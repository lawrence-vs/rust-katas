//! A command parser's keyword table and a worker-pool size, both shared by the whole process.
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Start,
    Stop,
    Pause,
    Resume,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    ZeroWorkers,
    AlreadySet { current: usize },
}

pub const DEFAULT_WORKERS: usize = 4;

static TABLE_BUILDS: AtomicUsize = AtomicUsize::new(0);

/// How many times the keyword table has been built in this process.
pub fn table_builds() -> usize {
    TABLE_BUILDS.load(Ordering::SeqCst)
}

fn build_table() -> HashMap<&'static str, Keyword> {
    TABLE_BUILDS.fetch_add(1, Ordering::SeqCst);
    HashMap::from([
        ("start", Keyword::Start),
        ("stop", Keyword::Stop),
        ("pause", Keyword::Pause),
        ("resume", Keyword::Resume),
        ("status", Keyword::Status),
    ])
}

/// The keyword for `word`, ignoring ASCII case.
/// CLUE: How many times does `build_table` run when 8 threads each look up 1,000 words? What kind of item lives as long as the process and is shared by every thread?
pub fn keyword(word: &str) -> Option<Keyword> {
    let table = build_table();
    table.get(word.to_ascii_lowercase().as_str()).copied()
}

static mut WORKERS: usize = 0;
static mut CONFIGURED: bool = false;

/// Sets the worker count once. Zero is rejected first; a second set reports the value already there.
/// CLUE: Two threads can both read `CONFIGURED == false` before either writes `true`. What does each caller get back, and what does `workers()` return afterwards?
pub fn set_workers(n: usize) -> Result<(), ConfigError> {
    if n == 0 {
        return Err(ConfigError::ZeroWorkers);
    }
    unsafe {
        if CONFIGURED {
            return Err(ConfigError::AlreadySet { current: WORKERS });
        }
        WORKERS = n;
        CONFIGURED = true;
    }
    Ok(())
}

/// The configured worker count, or `DEFAULT_WORKERS`; the first read locks the value in.
/// CLUE: "Check the flag, then write" is two steps. Which std type does both as one step for a single value, and also fills in a default on the first read?
pub fn workers() -> usize {
    unsafe {
        if !CONFIGURED {
            WORKERS = DEFAULT_WORKERS;
            CONFIGURED = true;
        }
        WORKERS
    }
}
