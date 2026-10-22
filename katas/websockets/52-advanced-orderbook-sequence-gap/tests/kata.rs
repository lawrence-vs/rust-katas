use kata_websockets_52::{
    BookError, Change, Delta, LocalBook, Outcome, Price, Qty, Side, Snapshot, SnapshotSource,
};
use std::collections::VecDeque;

struct FakeSource {
    snapshots: VecDeque<Snapshot>,
    calls: usize,
}

impl SnapshotSource for FakeSource {
    fn snapshot(&mut self) -> Snapshot {
        self.calls += 1;
        self.snapshots.pop_front().expect("no snapshot queued")
    }
}

fn levels(raw: &[(u64, u64)]) -> Vec<(Price, Qty)> {
    raw.iter().map(|&(p, q)| (Price(p), Qty(q))).collect()
}

fn snap(seq: u64, bids: &[(u64, u64)], asks: &[(u64, u64)]) -> Snapshot {
    Snapshot {
        seq,
        bids: levels(bids),
        asks: levels(asks),
    }
}

fn bid(p: u64, q: u64) -> Change {
    Change {
        side: Side::Bid,
        price: Price(p),
        qty: Qty(q),
    }
}

fn ask(p: u64, q: u64) -> Change {
    Change {
        side: Side::Ask,
        price: Price(p),
        qty: Qty(q),
    }
}

fn delta(seq: u64, changes: Vec<Change>) -> Delta {
    Delta { seq, changes }
}

fn book(snapshots: Vec<Snapshot>) -> LocalBook<FakeSource> {
    LocalBook::new(FakeSource {
        snapshots: snapshots.into(),
        calls: 0,
    })
}

#[test]
fn the_first_delta_loads_a_snapshot() {
    let mut b = book(vec![snap(10, &[(100, 5)], &[(101, 5)])]);
    assert_eq!(
        b.on_delta(delta(11, vec![bid(100, 6)])),
        Ok(Outcome::Resynced { snapshot: 10 }),
        "the first delta loads a snapshot"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(100, 6)]),
        "the first delta loads a snapshot"
    );
    assert_eq!(b.seq(), Some(11), "the first delta loads a snapshot");
    assert_eq!(b.source().calls, 1, "the first delta loads a snapshot");
}

#[test]
fn deltas_in_order_are_applied() {
    let mut b = book(vec![snap(10, &[(100, 5)], &[(101, 5)])]);
    b.on_delta(delta(11, vec![bid(100, 6)])).ok();
    assert_eq!(
        b.on_delta(delta(12, vec![bid(99, 2)])),
        Ok(Outcome::Applied),
        "deltas in order are applied"
    );
    assert_eq!(
        b.on_delta(delta(13, vec![ask(101, 9)])),
        Ok(Outcome::Applied),
        "deltas in order are applied"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(100, 6), (99, 2)]),
        "deltas in order are applied"
    );
    assert_eq!(
        b.levels(Side::Ask),
        levels(&[(101, 9)]),
        "deltas in order are applied"
    );
    assert_eq!(b.source().calls, 1, "deltas in order are applied");
}

#[test]
fn a_qty_of_zero_removes_the_level() {
    let mut b = book(vec![snap(10, &[(100, 5)], &[(101, 5)])]);
    b.on_delta(delta(11, vec![ask(101, 0)])).ok();
    assert_eq!(
        b.levels(Side::Ask),
        levels(&[]),
        "a qty of zero removes the level"
    );
}

#[test]
fn a_gap_triggers_a_resync() {
    let mut b = book(vec![
        snap(10, &[(100, 5)], &[(101, 5)]),
        snap(13, &[(99, 4)], &[(101, 3)]),
    ]);
    b.on_delta(delta(11, vec![bid(100, 6)])).ok();
    // delta 12 was lost
    assert_eq!(
        b.on_delta(delta(13, vec![ask(101, 3)])),
        Ok(Outcome::Resynced { snapshot: 13 }),
        "a gap triggers a resync"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(99, 4)]),
        "a gap triggers a resync"
    );
    assert_eq!(
        b.levels(Side::Ask),
        levels(&[(101, 3)]),
        "a gap triggers a resync"
    );
    assert_eq!(b.seq(), Some(13), "a gap triggers a resync");
    assert_eq!(b.source().calls, 2, "a gap triggers a resync");
}

#[test]
fn a_resync_drops_the_old_book() {
    let mut b = book(vec![
        snap(10, &[(100, 5), (98, 2)], &[(101, 5)]),
        snap(13, &[(97, 1)], &[(102, 4)]),
    ]);
    b.on_delta(delta(11, vec![bid(98, 3)])).ok();
    b.on_delta(delta(13, vec![])).ok();
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(97, 1)]),
        "a resync drops the old book"
    );
    assert_eq!(
        b.levels(Side::Ask),
        levels(&[(102, 4)]),
        "a resync drops the old book"
    );
}

#[test]
fn a_stale_delta_is_ignored() {
    let mut b = book(vec![snap(10, &[(100, 5)], &[])]);
    b.on_delta(delta(11, vec![bid(100, 7)])).ok();
    b.on_delta(delta(12, vec![bid(100, 9)])).ok();
    assert_eq!(
        b.on_delta(delta(11, vec![bid(100, 7)])),
        Ok(Outcome::Stale),
        "a stale delta is ignored"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(100, 9)]),
        "a stale delta is ignored"
    );
    assert_eq!(b.seq(), Some(12), "a stale delta is ignored");
    assert_eq!(b.source().calls, 1, "a stale delta is ignored");
}

#[test]
fn deltas_already_in_the_snapshot_are_skipped() {
    let mut b = book(vec![snap(10, &[(100, 5)], &[])]);
    assert_eq!(
        b.on_delta(delta(9, vec![bid(100, 1)])),
        Ok(Outcome::Resynced { snapshot: 10 }),
        "deltas already in the snapshot are skipped"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[(100, 5)]),
        "deltas already in the snapshot are skipped"
    );
    assert_eq!(
        b.seq(),
        Some(10),
        "deltas already in the snapshot are skipped"
    );
    assert_eq!(
        b.on_delta(delta(10, vec![bid(100, 1)])),
        Ok(Outcome::Stale),
        "deltas already in the snapshot are skipped"
    );
    assert_eq!(
        b.on_delta(delta(11, vec![bid(100, 6)])),
        Ok(Outcome::Applied),
        "deltas already in the snapshot are skipped"
    );
}

#[test]
fn a_snapshot_behind_the_stream_is_not_used() {
    let mut b = book(vec![
        snap(10, &[(100, 5)], &[(101, 5)]),
        snap(12, &[(100, 8)], &[(101, 8)]),
        snap(15, &[(100, 9)], &[(101, 9)]),
    ]);
    assert_eq!(
        b.on_delta(delta(11, vec![])),
        Ok(Outcome::Resynced { snapshot: 10 }),
        "a snapshot behind the stream is not used"
    );
    assert_eq!(
        b.on_delta(delta(14, vec![])),
        Err(BookError::SnapshotBehind {
            snapshot: 12,
            delta: 14
        }),
        "a snapshot behind the stream is not used"
    );
    assert_eq!(
        b.levels(Side::Bid),
        levels(&[]),
        "a snapshot behind the stream is not used"
    );
    assert_eq!(
        b.levels(Side::Ask),
        levels(&[]),
        "a snapshot behind the stream is not used"
    );
    assert_eq!(b.seq(), None, "a snapshot behind the stream is not used");
    assert_eq!(
        b.on_delta(delta(15, vec![])),
        Ok(Outcome::Resynced { snapshot: 15 }),
        "a snapshot behind the stream is not used"
    );
    assert_eq!(
        b.source().calls,
        3,
        "a snapshot behind the stream is not used"
    );
}
