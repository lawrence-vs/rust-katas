use std::time::Duration;

use kata_cache_172::{Decision, Limit, MemoryStore, Store, Ttl, check};

const LIMIT: Limit = Limit {
    max: 10,
    window: Duration::from_secs(60),
};
const USER: &str = "user-1";

/// The client dies right after INCR: the next PEXPIRE never reaches Redis.
struct LosesNextPexpire {
    inner: MemoryStore,
    lose: bool,
}

impl Store for LosesNextPexpire {
    fn time(&mut self) -> Duration {
        self.inner.time()
    }

    fn get(&mut self, key: &str) -> Option<i64> {
        self.inner.get(key)
    }

    fn incr(&mut self, key: &str) -> i64 {
        self.inner.incr(key)
    }

    fn pexpire(&mut self, key: &str, ttl: Duration) -> bool {
        if self.lose {
            self.lose = false;
            return false;
        }
        self.inner.pexpire(key, ttl)
    }

    fn pttl(&mut self, key: &str) -> Ttl {
        self.inner.pttl(key)
    }

    fn incr_px(&mut self, key: &str, ttl: Duration) -> i64 {
        self.inner.incr_px(key, ttl)
    }
}

fn allowed<S: Store>(store: &mut S, user: &str, times: usize) -> usize {
    (0..times)
        .filter(|_| check(store, user, LIMIT) == Decision::Allowed)
        .count()
}

#[test]
fn memory_store_behaves_like_redis() {
    let mut s = MemoryStore::new();

    assert_eq!(s.time(), Duration::ZERO);
    assert_eq!(s.get("a"), None);
    assert_eq!(s.pttl("a"), Ttl::Missing);
    assert!(!s.pexpire("a", Duration::from_secs(5)));

    assert_eq!(s.incr("a"), 1);
    assert_eq!(s.incr("a"), 2);
    assert_eq!(s.get("a"), Some(2));
    assert_eq!(s.pttl("a"), Ttl::Persistent);
    assert_eq!(s.persistent_keys(), vec!["a".to_string()]);

    assert!(s.pexpire("a", Duration::from_secs(5)));
    assert_eq!(s.pttl("a"), Ttl::Expires(Duration::from_secs(5)));
    assert!(s.persistent_keys().is_empty());

    s.advance(Duration::from_millis(4_999));
    assert_eq!(s.get("a"), Some(2));
    s.advance(Duration::from_millis(1));
    assert_eq!(s.get("a"), None);
    assert_eq!(s.pttl("a"), Ttl::Missing);

    assert_eq!(s.incr("a"), 1);
    assert_eq!(s.pttl("a"), Ttl::Persistent);

    assert_eq!(s.incr_px("b", Duration::from_secs(10)), 1);
    assert_eq!(s.pttl("b"), Ttl::Expires(Duration::from_secs(10)));
    s.advance(Duration::from_secs(4));
    assert_eq!(s.incr("b"), 2);
    assert_eq!(s.incr_px("b", Duration::from_secs(10)), 3);
    assert_eq!(s.pttl("b"), Ttl::Expires(Duration::from_secs(6)));
    assert_eq!(s.time(), Duration::from_secs(9));
}

#[test]
fn allows_up_to_the_limit_then_limits() {
    let mut s = MemoryStore::new();

    assert_eq!(allowed(&mut s, USER, 10), 10);
    assert_eq!(check(&mut s, USER, LIMIT), Decision::Limited);
}

#[test]
fn users_are_limited_separately() {
    let mut s = MemoryStore::new();

    assert_eq!(allowed(&mut s, "user-1", 12), 10);
    assert_eq!(allowed(&mut s, "user-2", 12), 10);
}

#[test]
fn every_counter_gets_its_ttl_in_the_same_step() {
    let mut s = LosesNextPexpire {
        inner: MemoryStore::new(),
        lose: true,
    };

    assert_eq!(check(&mut s, USER, LIMIT), Decision::Allowed);
    let persistent = s.inner.persistent_keys();
    assert!(
        persistent.is_empty(),
        "a rate-limit counter was left with no TTL: {persistent:?}"
    );
}

#[test]
fn a_lost_expire_does_not_block_a_user_forever() {
    let mut s = LosesNextPexpire {
        inner: MemoryStore::new(),
        lose: true,
    };

    assert_eq!(allowed(&mut s, USER, 11), 10);
    s.inner.advance(Duration::from_secs(600));
    assert_eq!(
        check(&mut s, USER, LIMIT),
        Decision::Allowed,
        "a user was blocked forever after a lost EXPIRE"
    );
}

#[test]
fn the_window_boundary_does_not_let_a_double_burst_through() {
    let mut s = MemoryStore::new();

    let early = allowed(&mut s, USER, 1);
    s.advance(Duration::from_millis(59_900));
    let late = allowed(&mut s, USER, 9);
    s.advance(Duration::from_millis(100));
    let boundary = allowed(&mut s, USER, 10);

    assert_eq!(early + late, 10);
    assert!(
        early + late + boundary <= 10,
        "the window boundary let {} requests through in 0.1s",
        late + boundary
    );
}

#[test]
fn the_previous_window_counts_in_proportion_to_its_overlap() {
    let mut s = MemoryStore::new();

    assert_eq!(allowed(&mut s, USER, 10), 10);
    s.advance(Duration::from_secs(90));
    assert_eq!(allowed(&mut s, USER, 10), 5);
}

#[test]
fn traffic_flows_again_once_a_full_window_has_passed() {
    let mut s = MemoryStore::new();

    assert_eq!(allowed(&mut s, USER, 10), 10);
    s.advance(Duration::from_secs(120));
    assert_eq!(allowed(&mut s, USER, 11), 10);
}

#[test]
fn refused_requests_do_not_count_against_the_next_window() {
    let mut s = MemoryStore::new();

    assert_eq!(allowed(&mut s, USER, 10), 10);
    s.advance(Duration::from_secs(60));
    assert_eq!(allowed(&mut s, USER, 10), 0);
    s.advance(Duration::from_secs(60));
    assert_eq!(allowed(&mut s, USER, 10), 10);
}
