use kata_smart_pointers_38::Node;
use std::rc::Rc;

fn ids_from(start: &Rc<Node>) -> Vec<u32> {
    let mut ids = Vec::new();
    Node::for_each_reachable(start, &mut |n| ids.push(n.id));
    ids
}

#[test]
fn a_chain_visits_each_node_once() {
    // a -> b -> c
    let a = Node::new(1, 1);
    let b = Node::new(2, 2);
    let c = Node::new(3, 3);
    Node::connect(&a, &b);
    Node::connect(&b, &c);

    assert_eq!(ids_from(&a), [1, 2, 3]);
    assert_eq!(Node::sum_all(&a), 6);
    Node::increment_all(&a);
    assert_eq!([a.value.get(), b.value.get(), c.value.get()], [2, 3, 4]);
}

#[test]
fn a_diamond_counts_the_shared_node_once() {
    // a -> b -> d and a -> c -> d
    let a = Node::new(1, 1);
    let b = Node::new(2, 10);
    let c = Node::new(3, 100);
    let d = Node::new(4, 1000);
    Node::connect(&a, &b);
    Node::connect(&a, &c);
    Node::connect(&b, &d);
    Node::connect(&c, &d);

    assert_eq!(Rc::strong_count(&d), 3);
    assert_eq!(ids_from(&a), [1, 2, 4, 3]);
    assert_eq!(Node::sum_all(&a), 1111);
    Node::increment_all(&a);
    assert_eq!(d.value.get(), 1001);
}

#[test]
fn a_cycle_ends() {
    // a -> b -> a
    let a = Node::new(1, 1);
    let b = Node::new(2, 2);
    Node::connect(&a, &b);
    Node::connect(&b, &a);

    assert_eq!(ids_from(&a), [1, 2]);
    assert_eq!(Node::sum_all(&a), 3);
    Node::increment_all(&a);
    assert_eq!([a.value.get(), b.value.get()], [2, 3]);

    // An Rc cycle never drops on its own; break it.
    b.edges.borrow_mut().clear();
}

#[test]
fn a_visitor_can_add_edges() {
    let a = Node::new(1, 0);
    let b = Node::new(2, 0);
    Node::connect(&a, &b);
    let audit = Node::new(9, 0);

    Node::for_each_reachable(&a, &mut |n| {
        if n.id != audit.id {
            Node::connect(n, &audit);
        }
    });

    assert!(a.edges.borrow().iter().any(|e| e.id == 9));
    assert!(b.edges.borrow().iter().any(|e| e.id == 9));
}
