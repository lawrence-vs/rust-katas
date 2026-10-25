use std::time::Duration;

use kata_cache_173::{
    CACHE_TTL, Cache, Db, Lookup, MemoryCache, MemoryDb, Row, Ttl, cache_key, fill, lookup, read,
    write,
};

const ID: &str = "42";

fn row(version: u64, value: &str) -> Row {
    Row {
        version,
        value: value.to_string(),
    }
}

fn setup() -> (MemoryCache, MemoryDb) {
    let mut db = MemoryDb::new();
    db.upsert(ID, "alice");
    (MemoryCache::new(), db)
}

#[test]
fn memory_db_behaves_like_a_database() {
    let mut db = MemoryDb::new();

    assert_eq!(db.select("x"), None);
    assert_eq!(db.upsert("x", "a"), row(1, "a"));
    assert_eq!(db.upsert("x", "b"), row(2, "b"));
    assert_eq!(db.select("x"), Some(row(2, "b")));
    assert_eq!(db.selects(), 2);
}

#[test]
fn memory_cache_behaves_like_redis() {
    let mut c = MemoryCache::new();
    let r1 = row(1, "a");
    let r2 = row(2, "b");
    let r3 = row(3, "c");

    assert_eq!(c.get("k"), None);
    assert_eq!(c.pttl("k"), Ttl::Missing);
    assert!(!c.del("k"));

    c.set("k", &r1);
    assert_eq!(c.get("k"), Some(r1.clone()));
    assert_eq!(c.pttl("k"), Ttl::Persistent);
    assert_eq!(c.persistent_keys(), vec!["k".to_string()]);

    c.set_px("k", &r1, Duration::from_secs(5));
    assert_eq!(c.pttl("k"), Ttl::Expires(Duration::from_secs(5)));
    assert!(c.persistent_keys().is_empty());

    c.advance(Duration::from_millis(4_999));
    assert_eq!(c.get("k"), Some(r1.clone()));
    c.advance(Duration::from_millis(1));
    assert_eq!(c.get("k"), None);
    assert_eq!(c.pttl("k"), Ttl::Missing);

    assert!(c.set_if_newer("k", &r2, Duration::from_secs(10)));
    assert!(!c.set_if_newer("k", &r1, Duration::from_secs(10)));
    assert!(!c.set_if_newer("k", &r2, Duration::from_secs(10)));
    assert_eq!(c.get("k"), Some(r2.clone()));
    assert_eq!(c.pttl("k"), Ttl::Expires(Duration::from_secs(10)));

    assert!(c.set_if_newer("k", &r3, Duration::from_secs(10)));
    assert_eq!(c.get("k"), Some(r3));

    assert!(c.del("k"));
    assert_eq!(c.get("k"), None);
}

#[test]
fn a_miss_loads_from_the_database_then_hits_the_cache() {
    let (mut cache, mut db) = setup();

    assert_eq!(read(&mut cache, &mut db, ID), Some(row(1, "alice")));
    assert_eq!(read(&mut cache, &mut db, ID), Some(row(1, "alice")));
    assert_eq!(db.selects(), 1);
}

#[test]
fn a_missing_row_is_not_cached() {
    let (mut cache, mut db) = setup();

    assert_eq!(lookup(&mut cache, &mut db, "nope"), Lookup::Absent);
    assert_eq!(read(&mut cache, &mut db, "nope"), None);
    assert_eq!(cache.get(&cache_key("nope")), None);
}

#[test]
fn a_read_after_a_write_sees_the_new_value() {
    let (mut cache, mut db) = setup();

    assert_eq!(read(&mut cache, &mut db, ID), Some(row(1, "alice")));
    assert_eq!(write(&mut cache, &mut db, ID, "bob"), row(2, "bob"));
    assert_eq!(read(&mut cache, &mut db, ID), Some(row(2, "bob")));
}

#[test]
fn a_slow_reader_cannot_put_a_stale_row_back() {
    let (mut cache, mut db) = setup();

    let pending = match lookup(&mut cache, &mut db, ID) {
        Lookup::Loaded(p) => p,
        other => panic!("expected a miss that loaded the row, got {other:?}"),
    };
    assert_eq!(write(&mut cache, &mut db, ID, "bob"), row(2, "bob"));
    assert_eq!(fill(&mut cache, pending), row(1, "alice"));

    let seen = read(&mut cache, &mut db, ID);
    assert_eq!(
        seen,
        Some(row(2, "bob")),
        "a stale row stuck in the cache after a write: {seen:?}"
    );
}

#[test]
fn every_cache_entry_has_a_ttl() {
    let (mut cache, mut db) = setup();

    read(&mut cache, &mut db, ID);
    let persistent = cache.persistent_keys();
    assert!(
        persistent.is_empty(),
        "a cache entry was left with no TTL: {persistent:?}"
    );
    assert_eq!(cache.pttl(&cache_key(ID)), Ttl::Expires(CACHE_TTL));

    write(&mut cache, &mut db, ID, "bob");
    assert_eq!(
        cache.pttl(&cache_key(ID)),
        Ttl::Expires(CACHE_TTL),
        "a write left the cache key without a fresh TTL"
    );
}

#[test]
fn staleness_is_bounded_when_a_write_skips_the_cache() {
    let (mut cache, mut db) = setup();

    assert_eq!(read(&mut cache, &mut db, ID), Some(row(1, "alice")));
    db.upsert(ID, "bob");
    assert_eq!(read(&mut cache, &mut db, ID), Some(row(1, "alice")));

    cache.advance(CACHE_TTL);
    assert_eq!(
        read(&mut cache, &mut db, ID),
        Some(row(2, "bob")),
        "a stale row outlived its TTL"
    );
}

#[test]
fn two_writes_leave_the_newest_row() {
    let (mut cache, mut db) = setup();

    write(&mut cache, &mut db, ID, "bob");
    write(&mut cache, &mut db, ID, "carol");
    assert_eq!(read(&mut cache, &mut db, ID), Some(row(3, "carol")));
}
