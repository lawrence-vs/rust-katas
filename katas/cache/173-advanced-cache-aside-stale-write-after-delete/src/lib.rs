use std::collections::HashMap;
use std::time::Duration;

/// How long a cached row may live.
pub const CACHE_TTL: Duration = Duration::from_secs(60);

/// A database row. The database bumps `version` on every write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// CLUE: Two rows for one key: which is newer? The value can't tell you.
    pub version: u64,
    pub value: String,
}

/// What `PTTL` replies: -2, -1, or the milliseconds left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ttl {
    Missing,
    Persistent,
    Expires(Duration),
}

/// The database. Each method is one statement.
pub trait Db {
    /// `SELECT version, value FROM users WHERE id = ?`.
    fn select(&mut self, id: &str) -> Option<Row>;
    /// `INSERT ... ON CONFLICT DO UPDATE SET value = ?, version = version + 1`: a new row starts at version 1. Returns the row as written.
    fn upsert(&mut self, id: &str, value: &str) -> Row;
}

/// The cache. Each method is one Redis command, so each runs atomically.
pub trait Cache {
    /// `GET key`.
    fn get(&mut self, key: &str) -> Option<Row>;
    /// `SET key row`: no TTL.
    fn set(&mut self, key: &str, row: &Row);
    /// `SET key row PX ms`.
    fn set_px(&mut self, key: &str, row: &Row, ttl: Duration);
    /// `DEL key`: `true` if a live key was removed.
    fn del(&mut self, key: &str) -> bool;
    /// `PTTL key`.
    fn pttl(&mut self, key: &str) -> Ttl;
    /// One Lua script, run in one step: `SET key row PX ms` only when the key is missing or holds a lower version. `true` if it wrote.
    fn set_if_newer(&mut self, key: &str, row: &Row, ttl: Duration) -> bool;
}

/// An in-process database that counts its `select`s.
#[derive(Debug, Default)]
pub struct MemoryDb {
    rows: HashMap<String, Row>,
    selects: usize,
}

impl MemoryDb {
    pub fn new() -> Self {
        Self::default()
    }

    /// How many `select`s have run.
    pub fn selects(&self) -> usize {
        self.selects
    }
}

impl Db for MemoryDb {
    fn select(&mut self, id: &str) -> Option<Row> {
        self.selects = self.selects.saturating_add(1);
        self.rows.get(id).cloned()
    }

    fn upsert(&mut self, id: &str, value: &str) -> Row {
        let version = self.rows.get(id).map_or(1, |r| r.version.saturating_add(1));
        let row = Row {
            version,
            value: value.to_string(),
        };
        self.rows.insert(id.to_string(), row.clone());
        row
    }
}

#[derive(Debug)]
struct Entry {
    row: Row,
    expires_at: Option<Duration>,
}

/// An in-process Redis. Every method is one command, so it runs atomically. Time moves only when a test calls advance.
#[derive(Debug, Default)]
pub struct MemoryCache {
    now: Duration,
    entries: HashMap<String, Entry>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&mut self, by: Duration) {
        self.now = self.now.saturating_add(by);
    }

    /// Keys with no TTL, sorted: rows that never leave the cache.
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

    fn put(&mut self, key: &str, row: &Row, expires_at: Option<Duration>) {
        self.entries.insert(
            key.to_string(),
            Entry {
                row: row.clone(),
                expires_at,
            },
        );
    }
}

impl Cache for MemoryCache {
    fn get(&mut self, key: &str) -> Option<Row> {
        self.purge(key);
        self.entries.get(key).map(|e| e.row.clone())
    }

    fn set(&mut self, key: &str, row: &Row) {
        self.put(key, row, None);
    }

    fn set_px(&mut self, key: &str, row: &Row, ttl: Duration) {
        let expires_at = self.now.saturating_add(ttl);
        self.put(key, row, Some(expires_at));
    }

    fn del(&mut self, key: &str) -> bool {
        self.purge(key);
        self.entries.remove(key).is_some()
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

    fn set_if_newer(&mut self, key: &str, row: &Row, ttl: Duration) -> bool {
        let newer = self.get(key).is_none_or(|old| old.version < row.version);
        if newer {
            self.set_px(key, row, ttl);
        }
        newer
    }
}

/// The cache key for a user id.
pub fn cache_key(id: &str) -> String {
    format!("user:{id}")
}

/// A row loaded after a cache miss and not yet written to the cache. A test holds one across a write to play a slow reader.
#[derive(Debug, PartialEq, Eq)]
pub struct PendingFill {
    key: String,
    row: Row,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    Hit(Row),
    Loaded(PendingFill),
    Absent,
}

/// Step one of a read: the cache, then the database on a miss.
pub fn lookup<C: Cache, D: Db>(cache: &mut C, db: &mut D, id: &str) -> Lookup {
    let key = cache_key(id);
    if let Some(row) = cache.get(&key) {
        return Lookup::Hit(row);
    }
    match db.select(id) {
        Some(row) => Lookup::Loaded(PendingFill { key, row }),
        None => Lookup::Absent,
    }
}

/// Step two of a read: put the loaded row in the cache.
/// CLUE: This row was read a while ago. Is it still the newest the cache has seen, and how long may it stay?
pub fn fill<C: Cache>(cache: &mut C, pending: PendingFill) -> Row {
    cache.set(&pending.key, &pending.row);
    pending.row
}

/// A whole read with no pause between the two steps.
pub fn read<C: Cache, D: Db>(cache: &mut C, db: &mut D, id: &str) -> Option<Row> {
    match lookup(cache, db, id) {
        Lookup::Hit(row) => Some(row),
        Lookup::Loaded(pending) => Some(fill(cache, pending)),
        Lookup::Absent => None,
    }
}

/// CLUE: The key is gone now. What stops a reader that loaded the row before this write from putting it back?
pub fn write<C: Cache, D: Db>(cache: &mut C, db: &mut D, id: &str, value: &str) -> Row {
    let row = db.upsert(id, value);
    cache.del(&cache_key(id));
    row
}
