//! Fill quantities, summed on worker threads.

use std::rc::Rc;
use std::thread::{self, JoinHandle};

/// Starts a background audit that sums `fills` while the caller keeps its own handle to them.
/// CLUE: Read the E0277 note. Which trait does `thread::spawn` need on everything the closure captures, and what about `Rc`'s reference count breaks it?
/// CLUE: This handle is returned, so the thread may outlive this call. Which std pointer is `Rc` with an atomic count?
pub fn spawn_audit(fills: &Rc<Vec<u64>>) -> JoinHandle<u64> {
    let fills = Rc::clone(fills);
    thread::spawn(move || fills.iter().sum())
}

/// Sums `fills` on `workers` threads, one chunk each. A `workers` of 0 is treated as 1.
/// CLUE: Every thread here is joined before this returns. Does each one need to own the data, or could it borrow a chunk of `fills`?
/// CLUE: `std::thread::scope`: what lifetime does the closure of a scoped thread need?
pub fn parallel_total(fills: &[u64], workers: usize) -> u64 {
    let shared = Rc::new(fills.to_vec());
    let chunk = fills.len().div_ceil(workers.max(1)).max(1);
    let mut handles = Vec::new();
    for start in (0..fills.len()).step_by(chunk) {
        let shared = Rc::clone(&shared);
        handles.push(thread::spawn(move || {
            let end = (start + chunk).min(shared.len());
            shared[start..end].iter().sum::<u64>()
        }));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
