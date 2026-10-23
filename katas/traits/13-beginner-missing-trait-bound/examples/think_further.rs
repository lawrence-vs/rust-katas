//! 13 beginner: a missing trait bound
//!
//! Think further, from the kata README:
//! 1. The slice holds a NaN: `[f64::NAN, 1.0]` and `[1.0, f64::NAN]`. What does `largest` return for each, and is "it depends on the order" acceptable for a function named `largest`?
//! 2. Two items tie for largest. Does your `largest` return a reference to the first or the last one? Does `Iterator::max` agree, and when would the difference matter?
//! 3. Prices at an exchange are often not floats (kata 42). If the price type were an integer newtype, which comparison traits would you derive on it, and what would that let you use that `f64` can't?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::cmp::Ordering;

// Think further 1: for floats, partial_cmp is None when a NaN is involved; pick a rule and say it.
fn largest_float(values: &[f64]) -> Option<f64> {
    values.iter().copied().fold(None, |best, v| match best {
        None => Some(v),
        Some(b) => match v.partial_cmp(&b) {
            Some(Ordering::Greater) => Some(v),
            _ => Some(b),
        },
    })
}

// Think further 2: Iterator::max returns the last of equal maxima; this version keeps the first.
fn first_largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut best = iter.next()?;
    for v in iter {
        if v > best {
            best = v;
        }
    }
    Some(best)
}

fn main() {
    println!("floats: {:?}", largest_float(&[1.0, 3.5, 2.0]));
    println!("with NaN first: {:?}", largest_float(&[f64::NAN, 1.0]));
    println!("with NaN last: {:?}", largest_float(&[1.0, f64::NAN]));

    // Think further 3: total_cmp gives a total order, so NaN has a fixed place.
    let mut v = [2.0f64, f64::NAN, 1.0];
    v.sort_by(f64::total_cmp);
    println!("total_cmp sort: {v:?}");

    println!("first largest of ints: {:?}", first_largest(&[3, 9, 9, 1]));
}
