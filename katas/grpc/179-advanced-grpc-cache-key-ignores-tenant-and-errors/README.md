# 179 grpc (advanced): a cache key that ignores the tenant

**Concept:** A response cache key is an authorisation boundary. It must hold everything the reply varies on: the method, the whole request, and the metadata that names the caller's tenant. Cache only successful replies of read-only methods, and forward the caller's deadline on a miss.
**Source:** docs.rs [tonic 0.14.6 `Request::set_timeout`](https://docs.rs/tonic/0.14.6/tonic/struct.Request.html); [tower `Layer`](https://docs.rs/tower/latest/tower/trait.Layer.html); [gRPC over HTTP/2 (`PROTOCOL-HTTP2.md`)](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md).
**Fails with:** `tenant b got tenant a's cached item`
**Vault:** your note on cache keys as authorisation boundaries

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`CachedCatalogue`, `TENANT_HEADER`, `CACHE_TTL`, `MAX_ENTRIES`, `pb`) and the constants. In `src/`, no `unwrap`, `expect`, `panic!` or `unsafe`. No new crates. Do not add single-flight (kata 31) and do not change the TTL (kata 172).

## Before you fix it
1. Run `cargo test`. Which tests pass, and why do they pass?
2. What is the cache key made of today? Which field of the request is not in it?
3. A caller sets a 100 ms deadline and misses the cache. Which header carries the deadline, and does the request you forward still have it?

## Answer in the PR
1. What is in your key now, and which reply would leak without each part?
2. Why is caching an error wrong, and which errors (if any) could you cache?
3. How does your cache decide a method may be cached, and why an allowlist and not a denylist?
4. Where does the caller's deadline travel, and who enforces it on a miss?
5. What does a stale price cost the business after `SetPrice`, and how long can it last?

## Think further
1. tonic reports an expired deadline as `Cancelled`, not `DeadlineExceeded`. Why might that be, and does your caller care?
2. Is the protobuf encoding of one message always the same bytes? (map fields; unknown fields)
3. Should `SetPrice` invalidate entries, and which ones?
4. A missing tenant header and an empty one: do they share a key?

## In the wild
- gRPC requests are HTTP/2 `POST` (`grpc/grpc` `doc/PROTOCOL-HTTP2.md`), so HTTP caches and CDNs don't cache them by default.
- Envoy's HTTP cache filter follows HTTP caching rules; check its docs for which methods it caches.
- `idempotency_level = NO_SIDE_EFFECTS` in `google/protobuf/descriptor.proto` is the method-level marker for read-only calls.
- Review checklist: a key built from one field; a `Result` stored in a cache; `Request::new(req.into_inner())` on a path that needs the caller's metadata.

## Related concepts
### The HTTP `Vary` header
Tells a shared cache which request headers a reply depends on, so one user's reply is not served to another. gRPC has no `Vary` by default, so the client or proxy must do this itself.
```
Vary: Accept-Encoding, Authorization
```

### The `idempotency_level` method option
A protobuf method option that says whether a call has side effects. A cache can use it to decide what may be replayed, but the option is a declaration, not enforcement.
```proto
rpc GetItem (GetItemRequest) returns (Item) {
  option idempotency_level = NO_SIDE_EFFECTS;
}
```

### `moka::future::Cache` (snippet only, not a dependency)
A concurrent cache with a capacity bound and a time-to-live. Use it when you need a production cache rather than a `HashMap`.
```rust
let cache: moka::future::Cache<String, u64> = moka::future::Cache::builder()
    .max_capacity(16)
    .time_to_live(std::time::Duration::from_secs(30))
    .build();
```

### Request coalescing (see kata 31)
Several callers that miss on the same key share one backend call. Kata 31 covers it; this kata does not.
```rust
let v: &u64 = cell.get_or_init(|| async { 42 }).await;
```
