//! 35 beginner: iterate, don't index
//!
//! Think further, from the kata README:
//! 1. The total no longer fits in a `u64`. What does `.sum()` do in a debug test build, and in `--release`? How would you write it with `checked_add` and `try_fold`? (See kata 42.)
//! 2. You need the position too, e.g. "the index of the first sell over 100". Which adaptor gives you the index without a counter, and which consumer stops at the first match?
//! 3. You need each fill and the one after it (the change between consecutive fills). Is that a reason to go back to `fills[i + 1]`, or do `windows(2)` or `zip` cover it?
//! 4. `bought_qty` takes `&[Fill]`. What would `impl IntoIterator<Item = &Fill>` let callers pass that a slice can't, e.g. a `VecDeque` or a `BTreeMap`'s values?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
struct Fill {
    side: Side,
    qty: u64,
}

// Think further 1: checked_add in try_fold, so an overflowing total is None, not a wrap.
fn bought_qty_checked(fills: &[Fill]) -> Option<u64> {
    fills
        .iter()
        .filter(|f| f.side == Side::Buy)
        .try_fold(0u64, |total, f| total.checked_add(f.qty))
}

// Think further 2: enumerate gives the index, and position stops at the first match.
fn first_sell_index(fills: &[Fill]) -> Option<usize> {
    fills.iter().position(|f| f.side == Side::Sell)
}

fn main() {
    let fills = [
        Fill {
            side: Side::Buy,
            qty: 5,
        },
        Fill {
            side: Side::Sell,
            qty: 2,
        },
        Fill {
            side: Side::Buy,
            qty: 3,
        },
    ];

    let total: u64 = fills
        .iter()
        .filter(|f| f.side == Side::Buy)
        .map(|f| f.qty)
        .sum();
    println!("bought: {total}");
    println!("checked: {:?}", bought_qty_checked(&fills));
    println!(
        "overflow: {:?}",
        bought_qty_checked(&[
            Fill {
                side: Side::Buy,
                qty: u64::MAX
            },
            Fill {
                side: Side::Buy,
                qty: 1
            }
        ])
    );
    println!("first sell at index: {:?}", first_sell_index(&fills));

    // Think further 3: windows(2) gives each fill with the one after it, with no index arithmetic.
    let changes: Vec<i64> = fills
        .windows(2)
        .map(|w| w[1].qty as i64 - w[0].qty as i64)
        .collect();
    println!("changes between fills: {changes:?}");

    // Think further 4: taking IntoIterator lets a caller pass a Vec, a VecDeque, or any iterator of fills.
    let from_vec: u64 = fills
        .to_vec()
        .iter()
        .filter(|f| f.side == Side::Buy)
        .map(|f| f.qty)
        .sum();
    println!("from an owned Vec: {from_vec}");
}
