/// One thing you may take at most once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub weight: usize,
    pub value: u64,
}

/// Best total value of a subset of `items` whose total weight is <= `capacity`.
/// Each item is used at most once. The sum of chosen values fits in `u64`;
/// overflow is out of scope.
/// CLUE: Say out loud what `best[w]` means after item i. Which row does `best[w - weight]` belong to when you read it?
/// CLUE: Find a one-item input where the answer should be one copy. What does this code return?
/// CLUE: Three of your cases pass today. Which input would tell 0/1 from "as many as you like"?
pub fn max_value(items: &[Item], capacity: usize) -> u64 {
    let mut best = vec![0u64; capacity + 1];
    for item in items {
        for w in item.weight..=capacity {
            best[w] = best[w].max(best[w - item.weight] + item.value);
        }
    }
    best[capacity]
}
