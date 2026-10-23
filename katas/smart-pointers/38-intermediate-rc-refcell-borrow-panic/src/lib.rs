//! A directed graph of shared nodes: many edges may point at the same node, and edges may form cycles.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Node {
    pub id: u32,
    pub value: Cell<i32>,
    /// CLUE: `connect` only has a shared `&Rc<Node>`, yet it pushes here. What does `RefCell` check at runtime that the compiler can't check here?
    pub edges: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(id: u32, value: i32) -> Rc<Node> {
        Rc::new(Node {
            id,
            value: Cell::new(value),
            edges: RefCell::new(Vec::new()),
        })
    }

    /// Adds an edge `from -> to`. Ids are unique within a graph.
    pub fn connect(from: &Rc<Node>, to: &Rc<Node>) {
        from.edges.borrow_mut().push(Rc::clone(to));
    }

    /// Calls `f` once on every node reachable from `start` (including `start`), depth first in edge order.
    /// `f` may call `connect` on the node it is given.
    /// CLUE: How long does the guard in `edges` live? Name every call that runs while it is still alive.
    /// CLUE: Walk a -> b -> a in your head. What does the second visit to `a` try to borrow, and who still holds it?
    /// CLUE: Without the panic, what would stop a -> b -> a? What must the walk remember between calls?
    pub fn for_each_reachable(start: &Rc<Node>, f: &mut dyn FnMut(&Rc<Node>)) {
        let edges = start.edges.borrow_mut();
        f(start);
        for next in edges.iter() {
            Node::for_each_reachable(next, f);
        }
    }

    /// Adds 1 to every reachable node, each once.
    pub fn increment_all(start: &Rc<Node>) {
        Node::for_each_reachable(start, &mut |n| n.value.set(n.value.get() + 1));
    }

    /// Sums every reachable node's value, each once.
    pub fn sum_all(start: &Rc<Node>) -> i32 {
        let mut total = 0;
        Node::for_each_reachable(start, &mut |n| total += n.value.get());
        total
    }
}
