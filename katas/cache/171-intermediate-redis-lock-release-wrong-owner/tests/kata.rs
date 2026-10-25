use std::collections::HashSet;
use std::time::Duration;

use kata_cache_171::{MemoryStore, Released, Store, acquire, release};

const KEY: &str = "lock:report";
const TTL: Duration = Duration::from_secs(10);
const PAST_TTL: Duration = Duration::from_secs(11);

/// A store where the caller pauses right after a GET: the lock expires and another client takes it before the next command.
struct PauseAfterGet {
    inner: MemoryStore,
    armed: bool,
    intruded: bool,
}

impl Store for PauseAfterGet {
    fn set_nx_px(&mut self, key: &str, value: &str, ttl: Duration) -> bool {
        self.inner.set_nx_px(key, value, ttl)
    }

    fn get(&mut self, key: &str) -> Option<String> {
        let seen = self.inner.get(key);
        if self.armed {
            self.armed = false;
            self.inner.advance(PAST_TTL);
            self.intruded = self.inner.set_nx_px(key, "intruder", TTL);
        }
        seen
    }

    fn del(&mut self, key: &str) -> bool {
        self.inner.del(key)
    }

    fn del_if_eq(&mut self, key: &str, expected: &str) -> bool {
        self.inner.del_if_eq(key, expected)
    }
}

#[test]
fn memory_store_behaves_like_redis() {
    let mut s = MemoryStore::new();

    assert!(s.set_nx_px("k", "a", TTL));
    assert!(!s.set_nx_px("k", "b", TTL));
    assert_eq!(s.get("k").as_deref(), Some("a"));

    s.advance(Duration::from_millis(9_999));
    assert_eq!(s.get("k").as_deref(), Some("a"));
    s.advance(Duration::from_millis(1));
    assert_eq!(s.get("k"), None);

    assert!(!s.del("k"));
    assert!(s.set_nx_px("k", "b", TTL));
    assert!(!s.del_if_eq("k", "a"));
    assert_eq!(s.get("k").as_deref(), Some("b"));
    assert!(s.del_if_eq("k", "b"));
    assert_eq!(s.get("k"), None);

    assert!(s.set_nx_px("k", "c", TTL));
    assert!(s.del("k"));
    assert!(!s.del("k"));
}

#[test]
fn a_held_lock_cannot_be_taken() {
    let mut s = MemoryStore::new();

    let a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    assert!(acquire(&mut s, KEY, "worker-b", TTL).is_none());
    assert_eq!(a.key(), KEY);
    assert_eq!(s.get(KEY).as_deref(), Some(a.token().as_str()));
}

#[test]
fn a_released_lock_can_be_taken_again() {
    let mut s = MemoryStore::new();

    let a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    assert_eq!(release(&mut s, a), Released::Freed);
    assert_eq!(s.get(KEY), None);
    assert!(acquire(&mut s, KEY, "worker-b", TTL).is_some());
}

#[test]
fn an_expired_lock_can_be_taken_by_another_client() {
    let mut s = MemoryStore::new();

    let _a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    s.advance(PAST_TTL);
    assert!(acquire(&mut s, KEY, "worker-b", TTL).is_some());
}

#[test]
fn a_late_release_does_not_free_the_next_holders_lock() {
    let mut s = MemoryStore::new();

    let a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    // worker-a pauses (a GC, a slow disk) past its TTL
    s.advance(PAST_TTL);
    let b = acquire(&mut s, KEY, "worker-b", TTL).unwrap();

    let outcome = release(&mut s, a);
    assert_eq!(
        s.get(KEY).as_deref(),
        Some(b.token().as_str()),
        "a late release freed another client's lock"
    );
    assert_eq!(outcome, Released::NotHeld);
    assert!(acquire(&mut s, KEY, "worker-c", TTL).is_none());
    assert_eq!(release(&mut s, b), Released::Freed);
}

#[test]
fn the_same_owner_gets_a_new_token_on_every_acquire() {
    let mut s = MemoryStore::new();

    let first = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    s.advance(PAST_TTL);
    // worker-a restarted with the same name and took the lock again
    let second = acquire(&mut s, KEY, "worker-a", TTL).unwrap();

    assert_ne!(
        first.token(),
        second.token(),
        "two acquires got the same token"
    );
    assert_eq!(release(&mut s, first), Released::NotHeld);
    assert_eq!(s.get(KEY).as_deref(), Some(second.token().as_str()));
}

#[test]
fn tokens_are_unique_across_many_acquires() {
    let mut s = MemoryStore::new();
    let mut seen = HashSet::new();

    for _ in 0..100 {
        let lease = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
        seen.insert(lease.token().clone());
        assert_eq!(release(&mut s, lease), Released::Freed);
    }
    assert_eq!(seen.len(), 100);
}

#[test]
fn release_checks_and_deletes_in_one_step() {
    let mut s = PauseAfterGet {
        inner: MemoryStore::new(),
        armed: false,
        intruded: false,
    };

    let a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    s.armed = true;
    let outcome = release(&mut s, a);

    if s.intruded {
        assert_eq!(
            s.inner.get(KEY).as_deref(),
            Some("intruder"),
            "a check-then-delete release freed another client's lock"
        );
        assert_eq!(outcome, Released::NotHeld);
    } else {
        assert_eq!(outcome, Released::Freed);
        assert_eq!(s.inner.get(KEY), None);
    }
}

#[test]
fn a_release_after_expiry_with_no_new_holder_is_not_held() {
    let mut s = MemoryStore::new();

    let a = acquire(&mut s, KEY, "worker-a", TTL).unwrap();
    s.advance(PAST_TTL);
    assert_eq!(release(&mut s, a), Released::NotHeld);
}

#[test]
fn locks_on_different_keys_are_independent() {
    let mut s = MemoryStore::new();

    assert!(acquire(&mut s, "lock:a", "worker-a", TTL).is_some());
    assert!(acquire(&mut s, "lock:b", "worker-b", TTL).is_some());
    assert!(acquire(&mut s, "lock:a", "worker-b", TTL).is_none());
}
