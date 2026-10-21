use std::collections::HashMap;

/// How many fills happened at each price.
/// CLUE: For one price, how many times does this loop look `price` up in `counts`?
pub fn count_fills(prices: &[u64]) -> HashMap<u64, u32> {
    let mut counts = HashMap::new();
    for &price in prices {
        if counts.contains_key(&price) {
            *counts.get_mut(&price).unwrap() += 1;
        } else {
            counts.insert(price, 1);
        }
    }
    counts
}

/// The order ids resting at `price`, created empty the first time that price is seen.
/// CLUE: The compiler says `book` is still mutably borrowed at `insert`. Which borrow, and why does returning it make it last to the end of the function?
/// CLUE: What does `HashMap::entry` hand you, and which of its methods gives back a `&mut V` whether or not the key was already there?
pub fn level_mut(book: &mut HashMap<u64, Vec<u64>>, price: u64) -> &mut Vec<u64> {
    if let Some(level) = book.get_mut(&price) {
        return level;
    }
    book.insert(price, Vec::new());
    book.get_mut(&price).unwrap()
}
