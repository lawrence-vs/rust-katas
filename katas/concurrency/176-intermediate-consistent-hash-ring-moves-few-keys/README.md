# 176 concurrency (intermediate): a hash ring where adding a node moves few keys

**Concept:** `hash(key) % n` ties every key to n, so a node joining moves about n/(n+1) of the keys and cold-starts every cache. A hash ring with many points per node moves only the ~1/(n+1) the new node takes, keeps loads within a bound, and still puts one hot key on one node.
**Source:** Karger et al., "Consistent Hashing and Random Trees" (STOC 1997, pp. 654–663); DeCandia et al., "Dynamo" (SOSP 2007), section 4.2 "Partitioning Algorithm" (virtual nodes); kata 73. No Readwise highlight.
**Fails with:** `adding one node moved too many keys`
**Vault:** your note on partitioning

The tests route 20,000 keys over 10 nodes, with no threads or network. The bounds are deterministic because `hash64` is fixed.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`, and no crates. The rules:
- Adding or removing one of n nodes moves about 1/n of the keys.
- A key only moves to the node that joined, or away from the node that left.
- Every node's load stays within ±40% of a fair share.
- The same nodes give the same routing in any join order.

## Before you fix it
1. Run `cargo test`. Which three tests fail, and what does each one count?
2. Before writing code, predict how many of the 20,000 keys move from 10 to 11 nodes with `% n`.

## Answer in the PR
1. With `% n`, what fraction of keys moves from 10 to 11 nodes, and why?
2. In your fix, which keys move when a node joins, and to where? Why do no others move?
3. What do many points per node buy? What does 10 points instead of 160 cost, and what does 1000 cost?
4. The hot-key test passes with your fix too. Why can no key-to-node mapping spread one key's load, and what would?
5. What does your code do when two points hash to the same value, and why does it barely matter?

## Think further
1. Redis Cluster: 16384 hash slots, `HASH_SLOT = CRC16(key) mod 16384` (`CLUSTER KEYSLOT`), `{hash tags}`, and resharding one slot at a time with `CLUSTER SETSLOT` (`IMPORTING`, `MIGRATING`, `NODE`) and `MIGRATE`. Fixed slots mapped to nodes versus a ring.
2. Kafka's default partitioner hashes a record's key with murmur2 and takes it mod the partition count (a record with no key goes to a sticky partition). What does adding partitions do to per-key order (katas 73, 170)?
3. Envoy's `RING_HASH` (`minimum_ring_size`, default 1024) and `MAGLEV` (fixed table of 65537 entries, faster but moves more keys when a host is removed) load-balancer policies, and nginx `hash $key consistent`, which uses ketama (plain `hash $key` remaps most keys when a server joins).
4. Weighted nodes: more points for a bigger node.
5. A hot key needs replicas or key splitting (`hot-key#0..k` with fan-out reads) or a local cache, not a better hash.
6. See the sd-katas card on partitioning. Name the card only; the repo is private, so no link.

## In the wild
- Dynamo and Cassandra vnodes (`num_tokens`, default 16 in `cassandra.yaml`).
- memcached ketama clients: each server hashed to 100–200 points on a 0..2^32 continuum.
- Envoy ring hash and Maglev, and nginx `consistent`.

In a review, check for:
- `hash(key) % nodes.len()` in a router of stateful nodes.
- One point per node.
- `DefaultHasher` used for routing that must agree across processes.

## Related concepts
- **Rendezvous (highest random weight) hashing:** each key goes to the node with the highest score for that key. Main use: small node sets with no ring to maintain.
  ```rust
  nodes.iter().max_by_key(|n| hash64(format!("{n}/{key}").as_bytes()))
  ```
- **Jump consistent hash:** maps a key to a bucket in 0..n with no node names, for sharded storage that only grows at the end. Main use: numbered shards that are added in order.
  ```rust
  // key: u64 (mut), n: i64 buckets; returns the bucket in 0..n
  let (mut b, mut j) = (-1i64, 0i64);
  while j < n { b = j; key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
      j = ((b + 1) as f64 * ((1u64 << 31) as f64 / ((key >> 33) + 1) as f64)) as i64; }
  ```
- **Bounded-load consistent hashing:** a ring that caps each node at `ceil(1.25 × mean)` and moves overflow to the next node. Main use: keeping hot shards from taking everything.
  ```rust
  candidates_in_order(key).find(|n| load[n] < cap)
  ```
