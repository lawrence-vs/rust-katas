//! 40 intermediate: an associated type, not a generic parameter
//!
//! Think further, from the kata README:
//! 1. Put back `impl Feed<Quote> for TradeFeed` next to the `Trade` impl on the starting code. What does `drain(&mut feed).len()` do now, and why does it compile with only one impl?
//! 2. `Add<Rhs>` has a generic `Rhs` and an associated `Output`. Why does it want both? When would your `Feed` want a generic parameter as well?
//! 3. A feed that lends out `&[u8]` frames borrowed from its own buffer can't express that with `type Item;`. What would `type Item<'a> where Self: 'a;` (a GAT) change?
//! 4. `Feed` looks like `Iterator`. What would you gain by implementing `Iterator` instead, and what does `next_event` naming buy you?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// One answer per type, so Item is an associated type.
trait Feed {
    type Item;
    fn next_event(&mut self) -> Option<Self::Item>;
}

struct Trades {
    left: Vec<u64>,
}

struct Quotes {
    left: Vec<(u64, u64)>,
}

impl Feed for Trades {
    type Item = u64;
    fn next_event(&mut self) -> Option<u64> {
        self.left.pop()
    }
}

impl Feed for Quotes {
    type Item = (u64, u64);
    fn next_event(&mut self) -> Option<(u64, u64)> {
        self.left.pop()
    }
}

// Think further 1: one impl per feed; the compiler picks the right one from the type alone.
fn drain<F: Feed>(feed: &mut F) -> usize {
    let mut count = 0;
    while feed.next_event().is_some() {
        count += 1;
    }
    count
}

// Think further 4: a feed used as an Iterator gets every adapter (map, filter, take) for free.
impl Iterator for Trades {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.next_event()
    }
}

fn main() {
    let mut trades = Trades { left: vec![101, 102, 103] };
    let mut quotes = Quotes { left: vec![(99, 101), (100, 102)] };
    println!("trades drained: {}", drain(&mut trades));
    println!("quotes drained: {}", drain(&mut quotes));

    let mut fresh = Trades { left: vec![1, 2, 3, 4] };
    let doubled: Vec<u64> = fresh.by_ref().take(2).map(|t| t * 2).collect();
    println!("first two doubled via Iterator: {doubled:?}");
}
