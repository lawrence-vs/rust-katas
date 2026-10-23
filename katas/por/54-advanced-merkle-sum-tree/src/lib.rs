use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

/// A client account. `balance` is in minor units (cents).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub id: u64,
    pub balance: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub hash: Hash,
    pub sum: u64,
}

impl Node {
    /// Pads an odd level. It adds nothing to the total.
    pub const EMPTY: Node = Node {
        hash: [0; 32],
        sum: 0,
    };
}

/// Where the sibling sits next to the node being folded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub sibling: Node,
    pub side: Side,
}

/// An inclusion proof from the exchange. `path` runs from the leaf to the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub leaf: Node,
    pub path: Vec<Step>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PorError {
    Empty,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    LeafMismatch,
    Overflow,
    RootMismatch,
}

/// sha256(b"leaf" || id || balance), both big-endian.
pub fn leaf_hash(account: &Account) -> Hash {
    let mut h = Sha256::new();
    h.update(b"leaf");
    h.update(account.id.to_be_bytes());
    h.update(account.balance.to_be_bytes());
    h.finalize().into()
}

/// The hash of a parent node.
///
/// CLUE: Which field of `Node` could the exchange change without this hash noticing?
pub fn node_hash(left: &Node, right: &Node) -> Hash {
    let mut h = Sha256::new();
    h.update(b"node");
    h.update(left.hash);
    h.update(right.hash);
    h.finalize().into()
}

pub struct MerkleSumTree {
    levels: Vec<Vec<Node>>,
}

impl MerkleSumTree {
    /// Builds the tree over every account.
    ///
    /// CLUE: What is `(u64::MAX - 49)` plus `100` under `wrapping_add`, and whose debt did that hide?
    pub fn build(accounts: &[Account]) -> Result<Self, PorError> {
        if accounts.is_empty() {
            return Err(PorError::Empty);
        }
        let mut level: Vec<Node> = accounts
            .iter()
            .map(|a| Node {
                hash: leaf_hash(a),
                sum: a.balance,
            })
            .collect();
        let mut levels = Vec::new();
        while level.len() > 1 {
            let mut padded = level.clone();
            if padded.len() % 2 == 1 {
                padded.push(Node::EMPTY);
            }
            let parents: Vec<Node> = padded
                .chunks(2)
                .map(|pair| Node {
                    hash: node_hash(&pair[0], &pair[1]),
                    sum: pair[0].sum.wrapping_add(pair[1].sum),
                })
                .collect();
            levels.push(level);
            level = parents;
        }
        levels.push(level);
        Ok(MerkleSumTree { levels })
    }

    pub fn root(&self) -> Node {
        self.levels
            .last()
            .and_then(|l| l.first())
            .copied()
            .unwrap_or(Node::EMPTY)
    }

    /// The inclusion proof for the account at `index`, or `None` past the end.
    pub fn proof(&self, index: usize) -> Option<Proof> {
        let leaf = *self.levels.first()?.get(index)?;
        let mut path = Vec::new();
        let mut i = index;
        for level in &self.levels[..self.levels.len() - 1] {
            let (sibling_index, side) = if i.is_multiple_of(2) {
                (i + 1, Side::Right)
            } else {
                (i - 1, Side::Left)
            };
            let sibling = level.get(sibling_index).copied().unwrap_or(Node::EMPTY);
            path.push(Step { sibling, side });
            i /= 2;
        }
        Some(Proof { leaf, path })
    }
}

/// Run by the client: does `proof` put `account` under the published `root`?
///
/// CLUE: Which argument does this function never read?
///
/// CLUE: The proof comes from the exchange. Which of its numbers should a client trust?
pub fn verify(root: &Node, _account: &Account, proof: &Proof) -> Result<(), VerifyError> {
    let mut current = proof.leaf;
    for step in &proof.path {
        let (left, right) = match step.side {
            Side::Left => (step.sibling, current),
            Side::Right => (current, step.sibling),
        };
        current = Node {
            hash: node_hash(&left, &right),
            sum: left.sum.wrapping_add(right.sum),
        };
    }
    if current == *root {
        Ok(())
    } else {
        Err(VerifyError::RootMismatch)
    }
}
