//! A directed graph where multiple nodes can point to the same target.
//! Nodes carry mutable payloads that adjacent nodes update.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
    pub value: RefCell<i32>,
    pub edges: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(id: u32, value: i32) -> Rc<Self> {
        Rc::new(Node {
            id,
            value: RefCell::new(value),
            edges: RefCell::new(Vec::new()),
        })
    }

    /// Adds a directed edge from this node to `target`.
    /// CLUE: What does `Rc<Node>` let you do that `&Node` doesn't?
    pub fn add_edge(this: &Rc<Self>, target: &Rc<Self>) {
        this.edges.borrow_mut().push(Rc::clone(target));
    }

    /// Increments `value` on every node reachable from this one (including self).
    /// CLUE: `RefCell::borrow_mut` panics if another borrow is active. How long does the borrow from `self.value.borrow_mut()` last?
    /// CLUE: The recursive call borrows again. What scope keeps the first borrow alive?
    pub fn increment_all(&self) {
        let mut value = self.value.borrow_mut();
        *value += 1;
        for edge in self.edges.borrow().iter() {
            edge.increment_all();
        }
    }

    /// Sums `value` across all reachable nodes.
    /// CLUE: Same double-borrow shape as `increment_all`, but with `borrow()`.
    pub fn sum_all(&self) -> i32 {
        let my_value = *self.value.borrow();
        let mut total = my_value;
        for edge in self.edges.borrow().iter() {
            total += edge.sum_all();
        }
        total
    }
}
