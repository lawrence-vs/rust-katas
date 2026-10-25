//! Routes each key to a cache node.

use std::collections::BTreeSet;
use std::fmt;

/// FNV-1a over the bytes, then the murmur3 64-bit finalizer so near-identical inputs land far
/// apart. The same on every run, platform and Rust version.
pub fn hash64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    h ^= h >> 33;
    h
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingError {
    ZeroVnodes,
    DuplicateNode(String),
    UnknownNode(String),
}

impl fmt::Display for RingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RingError::ZeroVnodes => write!(f, "a ring needs at least one point per node"),
            RingError::DuplicateNode(n) => write!(f, "node {n} is already on the ring"),
            RingError::UnknownNode(n) => write!(f, "node {n} is not on the ring"),
        }
    }
}

impl std::error::Error for RingError {}

/// Routes keys to nodes. Adding or removing one of n nodes should move about 1/n of the keys.
#[derive(Debug, Clone)]
pub struct Ring {
    nodes: BTreeSet<String>,
}

impl Ring {
    /// `vnodes_per_node` is how many points each node gets.
    /// CLUE: One point per node cuts the keys into arcs of random length. How uneven are ten random arcs, and how many points per node make them even?
    pub fn new(vnodes_per_node: usize) -> Result<Self, RingError> {
        if vnodes_per_node == 0 {
            return Err(RingError::ZeroVnodes);
        }
        Ok(Ring {
            nodes: BTreeSet::new(),
        })
    }

    /// CLUE: When node 11 joins, which keys should it take, and from whom? Which keys have no reason to move at all?
    pub fn add_node(&mut self, node: &str) -> Result<(), RingError> {
        if !self.nodes.insert(node.to_string()) {
            return Err(RingError::DuplicateNode(node.to_string()));
        }
        Ok(())
    }

    pub fn remove_node(&mut self, node: &str) -> Result<(), RingError> {
        if !self.nodes.remove(node) {
            return Err(RingError::UnknownNode(node.to_string()));
        }
        Ok(())
    }

    /// The nodes, ascending.
    pub fn nodes(&self) -> Vec<&str> {
        self.nodes.iter().map(String::as_str).collect()
    }

    /// The node that owns `key`, or `None` when the ring is empty.
    /// CLUE: For h in 0..110, for how many does `h % 10` equal `h % 11`? What does that say about the keys that keep their node?
    pub fn node_for(&self, key: &str) -> Option<&str> {
        let len = u64::try_from(self.nodes.len()).ok()?;
        if len == 0 {
            return None;
        }
        let i = usize::try_from(hash64(key.as_bytes()) % len).ok()?;
        self.nodes.iter().nth(i).map(String::as_str)
    }
}
