use kata_collections_191::{DijkstraError, shortest_distances};

#[test]
fn shortest_distances_on_fixed_graph() {
    let edges = [
        (0, 1, 4),
        (0, 2, 1),
        (2, 1, 2),
        (1, 3, 1),
        (2, 3, 5),
        (3, 4, 3),
    ];
    assert_eq!(
        shortest_distances(6, &edges, 0),
        Ok(vec![Some(0), Some(3), Some(1), Some(4), Some(7), None])
    );
}

#[test]
fn unreachable_nodes_are_none() {
    assert_eq!(
        shortest_distances(3, &[(0, 1, 1)], 0),
        Ok(vec![Some(0), Some(1), None])
    );
}

#[test]
fn edges_are_directed() {
    assert_eq!(
        shortest_distances(2, &[(1, 0, 1)], 0),
        Ok(vec![Some(0), None])
    );
}

#[test]
fn zero_weight_edges() {
    let edges = [(0, 1, 0), (1, 2, 0), (0, 2, 5)];
    assert_eq!(
        shortest_distances(3, &edges, 0),
        Ok(vec![Some(0), Some(0), Some(0)])
    );
}

#[test]
fn stale_entries_are_skipped() {
    let edges = [(0, 1, 10), (0, 2, 1), (2, 1, 1), (1, 3, 1)];
    assert_eq!(
        shortest_distances(4, &edges, 0),
        Ok(vec![Some(0), Some(2), Some(1), Some(3)])
    );
}

#[test]
fn parallel_edges_and_self_loops() {
    let edges = [(0, 1, 5), (0, 1, 2), (1, 1, 1)];
    assert_eq!(shortest_distances(2, &edges, 0), Ok(vec![Some(0), Some(2)]));
}

#[test]
fn distance_overflow_is_an_error() {
    let edges = [(0, 1, u64::MAX), (1, 2, 1)];
    assert_eq!(
        shortest_distances(3, &edges, 0),
        Err(DijkstraError::Overflow)
    );
}

#[test]
fn max_single_edge_is_not_overflow() {
    assert_eq!(
        shortest_distances(2, &[(0, 1, u64::MAX)], 0),
        Ok(vec![Some(0), Some(u64::MAX)])
    );
}

#[test]
fn out_of_range_is_an_error() {
    assert_eq!(
        shortest_distances(3, &[], 9),
        Err(DijkstraError::NodeOutOfRange(9))
    );
    assert_eq!(
        shortest_distances(3, &[(0, 7, 1)], 0),
        Err(DijkstraError::NodeOutOfRange(7))
    );
    assert_eq!(
        shortest_distances(3, &[(8, 0, 1)], 0),
        Err(DijkstraError::NodeOutOfRange(8))
    );
}
