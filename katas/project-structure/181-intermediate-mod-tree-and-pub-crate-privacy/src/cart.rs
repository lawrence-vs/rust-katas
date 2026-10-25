pub mod total;

use self::total::line_total;
use crate::percent::Percent;

/// CLUE: total.rs reads these fields. Does it need them to be `pub`, or is being a child module of `cart` enough?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub unit_cents: u64,
    pub qty: u32,
}

#[derive(Debug, Default)]
pub struct Cart {
    pub lines: Vec<Line>,
    pub discount: Option<Percent>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart::default()
    }

    pub fn add(&mut self, unit_cents: u64, qty: u32) {
        self.lines.push(Line { unit_cents, qty });
    }

    pub fn set_discount(&mut self, discount: Percent) {
        self.discount = Some(discount);
    }

    pub fn total(&self) -> Option<u64> {
        let mut subtotal: u64 = 0;
        for line in &self.lines {
            subtotal = subtotal.checked_add(line_total(line)?)?;
        }
        let off = self.discount.map_or(0, |d| d.of(subtotal));
        subtotal.checked_sub(off)
    }
}
