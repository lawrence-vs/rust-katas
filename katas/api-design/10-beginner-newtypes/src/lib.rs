use std::collections::HashMap;

/// Moves `amount` cents from one account to another.
/// CLUE: All four arguments after `balances` are `u64`. What stops a caller swapping `from` and `amount`?
/// CLUE: What does `#[repr(transparent)]` promise about a one-field struct's layout?
pub fn transfer(balances: &mut HashMap<u64, u64>, from: u64, to: u64, amount: u64) {
    *balances.entry(from).or_default() -= amount;
    *balances.entry(to).or_default() += amount;
}
