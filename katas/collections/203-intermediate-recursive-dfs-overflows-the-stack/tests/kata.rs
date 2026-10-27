use kata_collections_203::{Graph, GraphError};

fn graph(n: usize, edges: &[(usize, usize)]) -> Graph {
    let mut g = Graph::new(n);
    for &(a, b) in edges {
        g.add_edge(a, b).expect("edge in range");
    }
    g
}

const CHAIN: usize = 1_000_000;

#[test]
fn single_node_visits_itself() {
    assert_eq!(graph(1, &[]).dfs_order(0), Ok(vec![0]));
}

#[test]
fn preorder_follows_edge_insertion_order() {
    let g = graph(4, &[(0, 1), (0, 2), (1, 2), (1, 3)]);
    assert_eq!(g.dfs_order(0), Ok(vec![0, 1, 2, 3]));
}

#[test]
fn diamond_visits_shared_node_once() {
    let g = graph(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
    assert_eq!(g.dfs_order(0), Ok(vec![0, 1, 3, 2]));
}

#[test]
fn cycle_visits_each_node_once() {
    let g = graph(3, &[(0, 1), (1, 2), (2, 0)]);
    assert_eq!(g.dfs_order(1), Ok(vec![1, 2, 0]));
}

#[test]
fn self_loop_is_harmless() {
    let g = graph(2, &[(0, 0), (0, 1)]);
    assert_eq!(g.dfs_order(0), Ok(vec![0, 1]));
}

#[test]
fn disconnected_nodes_are_not_reached() {
    let g = graph(5, &[(0, 1), (3, 4)]);
    assert_eq!(g.dfs_order(0), Ok(vec![0, 1]));
    assert_eq!(g.reachable(0, 4), Ok(false));
    assert_eq!(g.reachable(3, 4), Ok(true));
    assert_eq!(g.reachable(4, 3), Ok(false));
    assert_eq!(g.reachable(2, 2), Ok(true));
}

#[test]
fn out_of_range_is_an_error() {
    let mut g = Graph::new(5);
    assert_eq!(
        g.dfs_order(5),
        Err(GraphError::NodeOutOfRange { node: 5, len: 5 })
    );
    assert_eq!(
        g.add_edge(0, 9),
        Err(GraphError::NodeOutOfRange { node: 9, len: 5 })
    );
    assert_eq!(
        g.reachable(0, 7),
        Err(GraphError::NodeOutOfRange { node: 7, len: 5 })
    );
    assert_eq!(
        Graph::new(0).dfs_order(0),
        Err(GraphError::NodeOutOfRange { node: 0, len: 0 })
    );
    assert!(Graph::new(0).is_empty());
}

#[test]
fn chain_reachability_small() {
    let g = graph(4, &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(g.reachable(0, 3), Ok(true));
    assert_eq!(g.reachable(3, 0), Ok(false));
    assert_eq!(g.len(), 4);
}

#[test]
fn million_node_chain_on_default_stack() {
    let edges: Vec<(usize, usize)> = (0..CHAIN - 1).map(|i| (i, i + 1)).collect();
    let g = graph(CHAIN, &edges);
    // No stack_size: this thread gets the default stack, like any thread you spawn.
    let handle = std::thread::Builder::new()
        .name("dfs-chain".to_string())
        .spawn(move || (g.dfs_order(0), g.reachable(0, CHAIN - 1)))
        .expect("spawn");
    let (order, reach) = handle.join().expect("dfs thread finished");
    let order = order.expect("start in range");
    assert_eq!(order.len(), CHAIN);
    assert_eq!(order.first(), Some(&0));
    assert_eq!(order.last(), Some(&(CHAIN - 1)));
    assert_eq!(reach, Ok(true));
}
