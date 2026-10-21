use kata_smart_pointers_38::Node;

#[test]
fn single_node_increments() {
    let a = Node::new(1, 10);
    a.increment_all();
    assert_eq!(*a.value.borrow(), 11);
}

#[test]
fn shared_target_gets_incremented_twice() {
    // a -> b <- c (b is shared)
    let b = Node::new(2, 100);
    let mut a = Node::new(1, 1);
    let mut c = Node::new(3, 1);
    Node::add_edge(&mut a, &b);
    Node::add_edge(&mut c, &b);

    // Each path increments b once; total +2
    a.increment_all();
    c.increment_all();

    assert_eq!(*b.value.borrow(), 102);
}

#[test]
fn sum_all_counts_shared_node_once_per_path() {
    // a -> b <- c
    let b = Node::new(2, 10);
    let mut a = Node::new(1, 1);
    let mut c = Node::new(3, 2);
    Node::add_edge(&mut a, &b);
    Node::add_edge(&mut c, &b);

    // a sees 1 + 10 = 11; c sees 2 + 10 = 12
    assert_eq!(a.sum_all(), 11);
    assert_eq!(c.sum_all(), 12);
}

#[test]
fn cycle_detected_by_runtime_borrow_check() {
    // a -> b -> a (cycle)
    let mut a = Node::new(1, 1);
    let mut b = Node::new(2, 2);
    Node::add_edge(&mut a, &b);
    Node::add_edge(&mut b, &a);

    // This must NOT deadlock or double-borrow; it must complete.
    a.increment_all();
    assert_eq!(*a.value.borrow(), 2);
    assert_eq!(*b.value.borrow(), 3);
}
