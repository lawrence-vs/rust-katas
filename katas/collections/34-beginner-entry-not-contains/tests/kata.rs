use kata_collections_34::{count_fills, level_mut};
use std::collections::HashMap;

#[test]
fn no_prices_no_counts() {
    assert!(count_fills(&[]).is_empty());
}

#[test]
fn counts_each_price() {
    let counts = count_fills(&[100, 101, 100, 100]);
    assert_eq!(counts.get(&100), Some(&3));
    assert_eq!(counts.get(&101), Some(&1));
    assert_eq!(counts.len(), 2);
}

#[test]
fn new_level_starts_empty() {
    let mut book = HashMap::new();
    assert!(level_mut(&mut book, 100).is_empty());
    assert_eq!(book.len(), 1);
}

#[test]
fn existing_level_keeps_its_orders() {
    let mut book = HashMap::new();
    level_mut(&mut book, 100).push(1);
    level_mut(&mut book, 100).push(2);
    level_mut(&mut book, 101).push(3);
    assert_eq!(book[&100], vec![1, 2]);
    assert_eq!(book[&101], vec![3]);
}
