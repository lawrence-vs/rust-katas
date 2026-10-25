use std::collections::HashMap;
use std::time::Duration;

/// What `PTTL` replies: -2, -1, or the milliseconds left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ttl {
    Missing,
    Persistent,
    Expires(Duration),
}

/// The commands a Redis rate limiter needs. Each method is one Redis command.
pub trait Store {
    /// `TIME`: the server clock.
    fn time(&mut self) -> Duration;
    /// `GET key`: the live counter, or `None`.
    fn get(&mut self, key: &str) -> Option<i64>;
    /// `INCR key`: a missing key starts at 0 and gets no TTL; an existing key keeps its TTL.
    fn incr(&mut self, key: &str) -> i64;
    /// `PEXPIRE key ms`: `false` if the key is missing.
    fn pexpire(&mut self, key: &str, ttl: Duration) -> bool;
    /// `PTTL key`.
    fn pttl(&mut self, key: &str) -> Ttl;
    /// One Lua script, run in one step: INCR, and PEXPIRE when the result is 1.
    fn incr_px(&mut self, key: &str, ttl: Duration) -> i64;
}

#[derive(Debug)]
struct Entry {
    value: i64,
    expires_at: Option<Duration>,
}

/// An in-process Redis. Every method is one command, so it runs atomically. Time moves only when a test calls advance.
#[derive(Debug, Default)]
pub struct MemoryStore {
    now: Duration,
    entries: HashMap<String, Entry>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&mut self, by: Duration) {
        self.now = self.now.saturating_add(by);
    }

    /// Keys with no TTL, sorted: counters that never reset.
    pub fn persistent_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, e)| e.expires_at.is_none())
            .map(|(k, _)| k.clone())
            .collect();
        keys.sort();
        keys
    }

    fn purge(&mut self, key: &str) {
        let now = self.now;
        if self
            .entries
            .get(key)
            .is_some_and(|e| e.expires_at.is_some_and(|t| t <= now))
        {
            self.entries.remove(key);
        }
    }
}

impl Store for MemoryStore {
    fn time(&mut self) -> Duration {
        self.now
    }

    fn get(&mut self, key: &str) -> Option<i64> {
        self.purge(key);
        self.entries.get(key).map(|e| e.value)
    }

    fn incr(&mut self, key: &str) -> i64 {
        self.purge(key);
        let entry = self.entries.entry(key.to_string()).or_insert(Entry {
            value: 0,
            expires_at: None,
        });
        entry.value = entry.value.saturating_add(1);
        entry.value
    }

    fn pexpire(&mut self, key: &str, ttl: Duration) -> bool {
        self.purge(key);
        let expires_at = self.now.saturating_add(ttl);
        match self.entries.get_mut(key) {
            Some(entry) => {
                entry.expires_at = Some(expires_at);
                true
            }
            None => false,
        }
    }

    fn pttl(&mut self, key: &str) -> Ttl {
        self.purge(key);
        match self.entries.get(key) {
            None => Ttl::Missing,
            Some(Entry {
                expires_at: None, ..
            }) => Ttl::Persistent,
            Some(Entry {
                expires_at: Some(t),
                ..
            }) => Ttl::Expires(t.saturating_sub(self.now)),
        }
    }

    fn incr_px(&mut self, key: &str, ttl: Duration) -> i64 {
        let n = self.incr(key);
        if n == 1 {
            self.pexpire(key, ttl);
        }
        n
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allowed,
    Limited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit {
    pub max: u32,
    pub window: Duration,
}

/// CLUE: The process dies between these two commands. What TTL does the counter have, and when does it reset?
/// CLUE: Ten requests just before this key expires and ten just after: how many got through in one second?
pub fn check<S: Store>(store: &mut S, user: &str, limit: Limit) -> Decision {
    let key = format!("rl:{user}");
    let count = store.incr(&key);
    if count == 1 {
        store.pexpire(&key, limit.window);
    }
    if count <= i64::from(limit.max) {
        Decision::Allowed
    } else {
        Decision::Limited
    }
}
