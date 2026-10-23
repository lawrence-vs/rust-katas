//! 02 intermediate: moving out of a Vec
//!
//! Think further, from the kata README:
//! 1. What does the current indexing do on an empty inventory, and what return type would make that case explicit to the caller?
//! 2. The caller wants to drop `inventory` straight after calling `equip`. Which of the two fixes still compiles, and why?
//! 3. Suppose `equip` really should *take* the item, so the inventory shrinks. What would the signature need, and how do `Vec::remove` and `Vec::swap_remove` differ in cost and order?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug)]
struct Item(String);

// Think further 1: an empty inventory is a value the caller can see, not an index panic.
fn first_item(inventory: &[Item]) -> Option<&Item> {
    inventory.first()
}

// Think further 3: taking the item out of the Vec needs an owned Item and a mutable Vec.
fn equip(inventory: &mut Vec<Item>, index: usize) -> Option<Item> {
    if index < inventory.len() {
        Some(inventory.swap_remove(index))
    } else {
        None
    }
}

fn main() {
    let mut inventory = vec![Item("sword".into()), Item("bow".into()), Item("shield".into())];
    println!("first: {:?}", first_item(&inventory));

    // remove keeps order but shifts every later item; swap_remove is O(1) but reorders.
    let removed = inventory.remove(0);
    println!("removed {removed:?}, left {inventory:?}");

    println!("equipped: {:?}", equip(&mut inventory, 0));
    println!("after swap_remove: {inventory:?}");

    println!("empty: {:?}", first_item(&[]));
}
