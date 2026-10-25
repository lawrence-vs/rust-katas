use std::collections::HashMap;
use std::time::Duration;

/// The commands a Redis lock needs. Each method is one Redis command.
pub trait Store {
    /// `SET key value NX PX ttl`: stores the value only if the key is not live.
    fn set_nx_px(&mut self, key: &str, value: &str, ttl: Duration) -> bool;
    /// `GET key`: the live value, or `None`.
    fn get(&mut self, key: &str) -> Option<String>;
    /// `DEL key`: removes the key whatever its value is.
    fn del(&mut self, key: &str) -> bool;
    /// `DELEX key IFEQ expected`, or the Lua release script: removes the key only if it holds `expected`, in one step.
    fn del_if_eq(&mut self, key: &str, expected: &str) -> bool;
}

#[derive(Debug)]
struct Entry {
    value: String,
    expires_at: Duration,
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

    fn live(&self, key: &str) -> Option<&Entry> {
        self.entries.get(key).filter(|e| e.expires_at > self.now)
    }
}

impl Store for MemoryStore {
    fn set_nx_px(&mut self, key: &str, value: &str, ttl: Duration) -> bool {
        if self.live(key).is_some() {
            return false;
        }
        let expires_at = self.now.saturating_add(ttl);
        self.entries.insert(
            key.to_string(),
            Entry {
                value: value.to_string(),
                expires_at,
            },
        );
        true
    }

    fn get(&mut self, key: &str) -> Option<String> {
        self.live(key).map(|e| e.value.clone())
    }

    fn del(&mut self, key: &str) -> bool {
        let was_live = self.live(key).is_some();
        self.entries.remove(key);
        was_live
    }

    fn del_if_eq(&mut self, key: &str, expected: &str) -> bool {
        if self.live(key).is_some_and(|e| e.value == expected) {
            self.entries.remove(key);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(String);

impl Token {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lease {
    key: String,
    token: Token,
}

impl Lease {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Released {
    Freed,
    /// The key was gone or held by someone else: nothing was deleted.
    NotHeld,
}

/// CLUE: Two acquires by the same owner, one after the other's TTL ran out: what tells their leases apart?
pub fn acquire<S: Store>(store: &mut S, key: &str, owner: &str, ttl: Duration) -> Option<Lease> {
    let token = Token(owner.to_string());
    if store.set_nx_px(key, token.as_str(), ttl) {
        Some(Lease {
            key: key.to_string(),
            token,
        })
    } else {
        None
    }
}

/// CLUE: This lease's TTL may have run out while its holder was paused. Whose lock does this delete then?
/// CLUE: If you check the value first and delete second, what can happen between the two commands?
pub fn release<S: Store>(store: &mut S, lease: Lease) -> Released {
    if store.del(&lease.key) {
        Released::Freed
    } else {
        Released::NotHeld
    }
}
