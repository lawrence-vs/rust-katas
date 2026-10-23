use std::collections::VecDeque;

use kata_batching_49::{BatchError, Batcher, Fill, LiquidityProvider, Order, OrderId, Pair, Qty};

const BTC: Pair = Pair("BTC-USD");
const ETH: Pair = Pair("ETH-USD");

struct FakeLp {
    caps: VecDeque<u64>,
    calls: Vec<(Pair, Qty)>,
}

impl LiquidityProvider for FakeLp {
    fn fill(&mut self, pair: Pair, qty: Qty) -> Qty {
        self.calls.push((pair, qty));
        match self.caps.pop_front() {
            Some(cap) => Qty(qty.0.min(cap)),
            None => qty,
        }
    }
}

fn order(id: u64, pair: Pair, qty: u64) -> Order {
    Order {
        id: OrderId(id),
        pair,
        qty: Qty(qty),
    }
}

fn fill(id: u64, qty: u64) -> Fill {
    Fill {
        order: OrderId(id),
        qty: Qty(qty),
    }
}

fn batcher(caps: &[u64]) -> Batcher<FakeLp> {
    Batcher::new(FakeLp {
        caps: caps.iter().copied().collect(),
        calls: Vec::new(),
    })
}

#[test]
fn one_provider_call_per_pair() {
    let mut b = batcher(&[]);
    b.submit(order(1, BTC, 2)).unwrap();
    b.submit(order(2, ETH, 5)).unwrap();
    b.submit(order(3, BTC, 3)).unwrap();
    let fills = b.flush();
    assert_eq!(
        b.lp().calls,
        vec![(BTC, Qty(5)), (ETH, Qty(5))],
        "one provider call per pair"
    );
    assert_eq!(
        fills,
        vec![fill(1, 2), fill(3, 3), fill(2, 5)],
        "one provider call per pair"
    );
}

#[test]
fn a_partial_fill_requeues_the_remainder() {
    let mut b = batcher(&[7]);
    b.submit(order(1, BTC, 4)).unwrap();
    b.submit(order(2, BTC, 6)).unwrap();
    let fills = b.flush();
    assert_eq!(
        fills,
        vec![fill(1, 4), fill(2, 3)],
        "a partial fill requeues the remainder"
    );
    assert_eq!(
        b.pending(BTC),
        vec![(OrderId(2), Qty(3))],
        "a partial fill requeues the remainder"
    );
    assert_eq!(
        b.lp().calls,
        vec![(BTC, Qty(10))],
        "a partial fill requeues the remainder"
    );
}

#[test]
fn the_remainder_goes_out_first_next_flush() {
    let mut b = batcher(&[7]);
    b.submit(order(1, BTC, 4)).unwrap();
    b.submit(order(2, BTC, 6)).unwrap();
    b.flush();
    b.submit(order(3, BTC, 5)).unwrap();
    let fills = b.flush();
    assert_eq!(
        b.lp().calls[1],
        (BTC, Qty(8)),
        "the remainder goes out first next flush"
    );
    assert_eq!(
        fills,
        vec![fill(2, 3), fill(3, 5)],
        "the remainder goes out first next flush"
    );
    assert!(
        b.pending(BTC).is_empty(),
        "the remainder goes out first next flush"
    );
}

#[test]
fn nothing_filled_keeps_every_order() {
    let mut b = batcher(&[0]);
    b.submit(order(1, BTC, 4)).unwrap();
    b.submit(order(2, BTC, 6)).unwrap();
    let fills = b.flush();
    assert_eq!(fills, vec![], "nothing filled keeps every order");
    assert_eq!(
        b.pending(BTC),
        vec![(OrderId(1), Qty(4)), (OrderId(2), Qty(6))],
        "nothing filled keeps every order"
    );
    assert_eq!(b.lp().calls.len(), 1, "nothing filled keeps every order");
}

#[test]
fn an_empty_flush_calls_no_one() {
    let mut b = batcher(&[]);
    assert_eq!(b.flush(), vec![], "an empty flush calls no one");
    assert!(b.lp().calls.is_empty(), "an empty flush calls no one");
    b.submit(order(1, BTC, 2)).unwrap();
    b.flush();
    b.flush();
    assert_eq!(
        b.lp().calls,
        vec![(BTC, Qty(2))],
        "an empty flush calls no one"
    );
}

#[test]
fn a_zero_qty_order_is_rejected() {
    let mut b = batcher(&[]);
    assert_eq!(
        b.submit(order(1, BTC, 0)),
        Err(BatchError::ZeroQty),
        "a zero qty order is rejected"
    );
    assert!(b.pending(BTC).is_empty(), "a zero qty order is rejected");
}

#[test]
fn a_pair_total_past_u64_max_is_rejected() {
    let mut b = batcher(&[]);
    b.submit(order(1, BTC, u64::MAX)).unwrap();
    assert_eq!(
        b.submit(order(2, BTC, 1)),
        Err(BatchError::PairOverflow { pair: BTC }),
        "a pair total past u64 max is rejected"
    );
    assert_eq!(
        b.submit(order(3, ETH, 1)),
        Ok(()),
        "a pair total past u64 max is rejected"
    );
    assert_eq!(
        b.pending(BTC),
        vec![(OrderId(1), Qty(u64::MAX))],
        "a pair total past u64 max is rejected"
    );
}
