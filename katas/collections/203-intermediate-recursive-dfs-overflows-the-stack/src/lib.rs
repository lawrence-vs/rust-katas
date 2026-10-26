use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
    /// `node` is not in `0..len`.
    NodeOutOfRange { node: usize, len: usize },
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::NodeOutOfRange { node, len } => {
                write!(f, "node {node} out of range for graph of {len} nodes")
            }
        }
    }
}

impl std::error::Error for GraphError {}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); nodes],
        }
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Directed edge `from -> to`. Checks `to` first, then `from`.
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
        let len = self.adj.len();
        if to >= len {
            return Err(GraphError::NodeOutOfRange { node: to, len });
        }
        match self.adj.get_mut(from) {
            Some(neighbours) => {
                neighbours.push(to);
                Ok(())
            }
            None => Err(GraphError::NodeOutOfRange { node: from, len }),
        }
    }

    /// Preorder from `start`; neighbours in insertion order; each node once.
    ///
    /// CLUE: A thread's stack is fixed when it starts (2 MiB by default). Where else can
    /// you keep the nodes still to visit, and how big can that grow?
    pub fn dfs_order(&self, start: usize) -> Result<Vec<usize>, GraphError> {
        let len = self.adj.len();
        if start >= len {
            return Err(GraphError::NodeOutOfRange { node: start, len });
        }
        let mut visited = vec![false; len];
        let mut order = Vec::new();
        self.visit(start, &mut visited, &mut order);
        Ok(order)
    }

    /// True if `to` can be reached from `from` (a node reaches itself).
    pub fn reachable(&self, from: usize, to: usize) -> Result<bool, GraphError> {
        let order = self.dfs_order(from)?;
        let len = self.adj.len();
        if to >= len {
            return Err(GraphError::NodeOutOfRange { node: to, len });
        }
        Ok(order.contains(&to))
    }

    /// CLUE: Each call here is one stack frame. How many frames are live at once on a
    /// chain of 1,000,000 nodes?
    ///
    /// CLUE: If you keep the pending nodes yourself, do you mark a node seen when you
    /// push it or when you pop it? Run the diamond and the self-loop tests both ways.
    fn visit(&self, node: usize, visited: &mut [bool], order: &mut Vec<usize>) {
        match visited.get_mut(node) {
            Some(seen) if !*seen => *seen = true,
            _ => return,
        }
        order.push(node);
        if let Some(next) = self.adj.get(node) {
            for &n in next {
                self.visit(n, visited, order);
            }
        }
    }
}
