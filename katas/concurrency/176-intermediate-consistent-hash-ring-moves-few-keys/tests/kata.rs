use std::collections::BTreeMap;

use kata_concurrency_176::{Ring, RingError, hash64};

const VNODES: usize = 160;
const KEYS: usize = 20_000;
const NEW: &str = "node-10";

fn keys() -> Vec<String> {
    (0..KEYS).map(|i| format!("user-{i}")).collect()
}

fn ring_of(n: usize) -> Ring {
    let mut ring = Ring::new(VNODES).expect("160 points per node is valid");
    for i in 0..n {
        ring.add_node(&format!("node-{i}"))
            .expect("each node name is new");
    }
    ring
}

fn owners(ring: &Ring, keys: &[String]) -> Vec<String> {
    keys.iter()
        .map(|k| {
            ring.node_for(k)
                .expect("a ring with nodes routes every key")
                .to_string()
        })
        .collect()
}

fn loads<'a>(owners: impl IntoIterator<Item = &'a str>) -> BTreeMap<String, usize> {
    let mut loads = BTreeMap::new();
    for o in owners {
        *loads.entry(o.to_string()).or_insert(0) += 1;
    }
    loads
}

#[test]
fn hash64_is_stable_and_spreads() {
    assert_eq!(hash64(b"user-1"), hash64(b"user-1"));
    assert_ne!(hash64(b"user-1"), hash64(b"user-2"));
}

#[test]
fn zero_points_per_node_is_an_error() {
    assert!(matches!(Ring::new(0), Err(RingError::ZeroVnodes)));
}

#[test]
fn an_empty_ring_routes_nowhere() {
    let ring = Ring::new(VNODES).expect("valid");
    assert!(ring.nodes().is_empty());
    assert_eq!(ring.node_for("user-1"), None);
}

#[test]
fn adding_or_removing_twice_is_an_error() {
    let mut ring = Ring::new(VNODES).expect("valid");
    assert_eq!(ring.add_node("a"), Ok(()));
    assert_eq!(
        ring.add_node("a"),
        Err(RingError::DuplicateNode("a".to_string()))
    );
    assert_eq!(ring.node_for("user-1"), Some("a"));
    assert_eq!(
        ring.remove_node("b"),
        Err(RingError::UnknownNode("b".to_string()))
    );
    assert_eq!(ring.remove_node("a"), Ok(()));
    assert!(ring.nodes().is_empty());
    assert_eq!(ring.node_for("user-1"), None);
}

#[test]
fn every_node_gets_a_fair_share() {
    let ring = ring_of(10);
    let owners = owners(&ring, &keys());
    let loads = loads(owners.iter().map(String::as_str));
    assert_eq!(loads.len(), 10, "a node got no keys: {loads:?}");
    for (node, load) in &loads {
        assert!(
            (1_200..=2_800).contains(load),
            "node {node} got {load} of {KEYS} keys; a fair share is 2000 ± 40%"
        );
    }
}

#[test]
fn adding_a_node_moves_about_one_nth_of_the_keys() {
    let keys = keys();
    let mut ring = ring_of(10);
    let before = owners(&ring, &keys);
    ring.add_node(NEW).expect("new node");
    let after = owners(&ring, &keys);
    let moved = before.iter().zip(&after).filter(|(b, a)| b != a).count();
    assert!(
        moved <= 3_000,
        "adding one node moved too many keys: {moved} of {KEYS}, expected about {}",
        KEYS / 11
    );
    assert!(
        moved >= 1_000,
        "the new node took too few keys: {moved} of {KEYS}"
    );
}

#[test]
fn keys_only_move_to_the_new_node() {
    let keys = keys();
    let mut ring = ring_of(10);
    let before = owners(&ring, &keys);
    ring.add_node(NEW).expect("new node");
    let after = owners(&ring, &keys);
    for ((k, b), a) in keys.iter().zip(&before).zip(&after) {
        assert!(
            b == a || a == NEW,
            "a key moved between two old nodes: {k} went from {b} to {a}"
        );
    }
}

#[test]
fn removing_a_node_moves_only_its_keys() {
    let keys = keys();
    let mut ring = ring_of(10);
    let before = owners(&ring, &keys);
    ring.remove_node("node-3").expect("node-3 is on the ring");
    let after = owners(&ring, &keys);
    for ((k, b), a) in keys.iter().zip(&before).zip(&after) {
        assert!(
            b == a || b == "node-3",
            "removing a node moved a key it did not own: {k} went from {b} to {a}"
        );
        assert_ne!(a, "node-3", "a key still routes to a removed node: {k}");
    }
}

#[test]
fn adding_then_removing_a_node_restores_every_key() {
    let keys = keys();
    let mut ring = ring_of(10);
    let before = owners(&ring, &keys);
    ring.add_node(NEW).expect("new node");
    ring.remove_node(NEW).expect("just added");
    assert_eq!(
        owners(&ring, &keys),
        before,
        "a key did not come back to its old node"
    );
}

#[test]
fn the_order_nodes_join_does_not_matter() {
    let keys = keys();
    let forward = ring_of(10);
    let mut backward = Ring::new(VNODES).expect("valid");
    for i in (0..10).rev() {
        backward.add_node(&format!("node-{i}")).expect("new node");
    }
    assert_eq!(forward.nodes(), backward.nodes());
    assert_eq!(forward.nodes()[0], "node-0");
    assert_eq!(
        owners(&forward, &keys),
        owners(&backward, &keys),
        "two rings with the same nodes disagree on a key"
    );
}

// Shown, not solved: no key-to-node map can split one key's traffic. See Think further.
#[test]
fn a_hot_key_still_overloads_its_node() {
    let ring = ring_of(10);
    let keys = keys();
    let mut requests: Vec<&str> = keys[..10_000].iter().map(String::as_str).collect();
    requests.extend(std::iter::repeat_n("hot-key", 5_000));
    let mut nodes = Vec::new();
    for r in &requests {
        nodes.push(ring.node_for(r).expect("routes").to_string());
    }
    let loads = loads(nodes.iter().map(String::as_str));
    let hot = ring.node_for("hot-key").expect("routes");
    let mean = requests.len() / 10;
    assert!(
        loads[hot] >= 3 * mean,
        "expected the hot key to overload its node: {} vs a mean of {mean}",
        loads[hot]
    );
}
