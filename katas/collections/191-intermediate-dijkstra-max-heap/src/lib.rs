use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub enum DijkstraError {
    NodeOutOfRange(usize),
    Overflow,
}

/// Shortest distance from `source` to every node, over directed weighted edges `(from, to, weight)`.
/// CLUE: `BinaryHeap::pop` returns the greatest item. Which distance do you want first?
/// CLUE: A path weight is a sum of untrusted u64s. What should adding past u64::MAX do here?
pub fn shortest_distances(
    n: usize,
    edges: &[(usize, usize, u64)],
    source: usize,
) -> Result<Vec<Option<u64>>, DijkstraError> {
    if source >= n {
        return Err(DijkstraError::NodeOutOfRange(source));
    }
    let mut adj: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n];
    for &(from, to, w) in edges {
        if from >= n {
            return Err(DijkstraError::NodeOutOfRange(from));
        }
        if to >= n {
            return Err(DijkstraError::NodeOutOfRange(to));
        }
        adj[from].push((to, w));
    }

    let mut dist: Vec<Option<u64>> = vec![None; n];
    let mut done = vec![false; n];
    dist[source] = Some(0);
    let mut heap = BinaryHeap::new();
    heap.push((0u64, source));

    while let Some((d, u)) = heap.pop() {
        if done[u] {
            continue;
        }
        done[u] = true;
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if dist[v].is_none_or(|cur| nd < cur) {
                dist[v] = Some(nd);
                heap.push((nd, v));
            }
        }
    }
    Ok(dist)
}
