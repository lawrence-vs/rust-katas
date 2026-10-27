use std::collections::BinaryHeap;

/// Ask prices waiting to be matched. The cheapest ask must come out first.
/// CLUE: `BinaryHeap` pops its greatest element first. Which way does a seller's best price point?
pub struct Asks {
    heap: BinaryHeap<u64>,
}

impl Asks {
    pub fn new() -> Self {
        Asks {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, price: u64) {
        self.heap.push(price);
    }

    pub fn pop_best(&mut self) -> Option<u64> {
        self.heap.pop()
    }
}

impl Default for Asks {
    fn default() -> Self {
        Self::new()
    }
}
