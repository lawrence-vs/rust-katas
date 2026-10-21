use kata_threads_41::{parallel_total, spawn_audit};
use std::sync::Arc;

#[test]
fn audit_runs_on_its_own_thread() {
    let fills = Arc::new(vec![3, 4, 5]);
    let audit = spawn_audit(&fills);
    assert_eq!(fills.len(), 3, "the caller keeps its handle");
    assert_eq!(audit.join().unwrap(), 12);
    assert_eq!(
        Arc::strong_count(&fills),
        1,
        "the thread drops its handle when it ends"
    );
}

#[test]
fn audit_outlives_the_callers_handle() {
    let fills = Arc::new(vec![1, 2, 3]);
    let audit = spawn_audit(&fills);
    drop(fills);
    assert_eq!(audit.join().unwrap(), 6);
}

#[test]
fn parallel_total_borrows_a_stack_array() {
    let fills = [10_u64; 1000];
    assert_eq!(parallel_total(&fills, 8), 10_000);
}

#[test]
fn parallel_total_edges() {
    assert_eq!(parallel_total(&[1, 2, 3, 4, 5], 2), 15);
    assert_eq!(parallel_total(&[7], 4), 7);
    assert_eq!(parallel_total(&[], 3), 0);
    assert_eq!(parallel_total(&[1, 2], 0), 3);
}
