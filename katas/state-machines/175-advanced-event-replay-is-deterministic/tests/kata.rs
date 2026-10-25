use kata_state_machines_175::{
    Change, DecodeError, Event, EventId, Outcome, ReplayError, Stock, replay,
};

fn ev(version: u64, at_ms: u64, change: Change) -> Event {
    Event {
        version,
        id: EventId(100 + version),
        at_ms,
        change,
    }
}

fn log() -> Vec<Event> {
    vec![
        ev(1, 1_000, Change::Received { qty: 10 }),
        ev(2, 2_000, Change::Shipped { qty: 3 }),
        ev(3, 3_000, Change::Received { qty: 5 }),
        ev(4, 4_000, Change::Shipped { qty: 7 }),
    ]
}

#[test]
fn encode_then_decode_round_trips() {
    let s = replay(&log()).expect("valid log");
    assert_eq!(Stock::decode(&s.encode()), Ok(s));
}

#[test]
fn decode_rejects_a_snapshot_of_the_wrong_length() {
    assert_eq!(
        Stock::decode(&[0u8; 31]),
        Err(DecodeError::WrongLength { got: 31 })
    );
    assert_eq!(
        Stock::decode(&[0u8; 33]),
        Err(DecodeError::WrongLength { got: 33 })
    );
    assert_eq!(Stock::decode(&[0u8; 32]), Ok(Stock::default()));
}

#[test]
fn an_empty_log_replays_to_the_default_state() {
    assert_eq!(replay(&[]), Ok(Stock::default()));
    assert_eq!(Stock::default().encode(), [0u8; 32]);
}

#[test]
fn shipping_more_than_on_hand_is_an_error_and_changes_nothing() {
    let mut s = replay(&log()[..1]).expect("valid log");
    let before = s.encode();
    let bad = ev(2, 2_000, Change::Shipped { qty: 11 });
    assert_eq!(
        s.apply(&bad),
        Err(ReplayError::Oversold {
            version: 2,
            on_hand: 10,
            qty: 11
        })
    );
    assert_eq!(s.encode(), before);
}

#[test]
fn replaying_the_same_log_twice_is_byte_equal() {
    let a = replay(&log()).map(|s| s.encode());
    let b = replay(&log()).map(|s| s.encode());
    assert_eq!(a, b, "two replays of one log disagree");
}

#[test]
fn state_takes_its_time_and_lot_from_the_events() {
    let s = replay(&log()).expect("valid log");
    assert_eq!(s.on_hand(), 5);
    assert_eq!(s.version(), 4);
    assert_eq!(
        s.updated_at_ms(),
        4_000,
        "apply read the clock instead of the event"
    );
    assert_eq!(
        s.last_lot(),
        103,
        "apply minted a lot instead of using the event id"
    );
}

#[test]
fn a_snapshot_plus_the_tail_equals_a_full_replay() {
    let events = log();
    let full = replay(&events).expect("valid log").encode();
    for n in 0..=events.len() {
        let head = replay(&events[..n]).expect("valid prefix");
        let mut s = Stock::decode(&head.encode()).expect("32 bytes");
        s.replay_from(&events[n..]).expect("valid tail");
        assert_eq!(
            s.encode(),
            full,
            "snapshot at version {n} plus the tail disagrees with a full replay"
        );
    }
}

#[test]
fn a_version_gap_is_an_error_and_changes_nothing() {
    let events = log();
    let mut s = replay(&events[..2]).expect("valid prefix");
    let before = s.encode();
    assert_eq!(
        s.apply(&events[3]),
        Err(ReplayError::Gap {
            expected: 3,
            got: 4
        })
    );
    assert_eq!(s.encode(), before);
    assert_eq!(
        replay(&[events[0], events[2]]),
        Err(ReplayError::Gap {
            expected: 2,
            got: 3
        })
    );
}

#[test]
fn a_duplicate_event_is_a_no_op() {
    let events = log();
    let mut s = replay(&events[..3]).expect("valid prefix");
    let before = s.encode();
    assert_eq!(s.apply(&events[2]), Ok(Outcome::Duplicate));
    assert_eq!(s.apply(&events[0]), Ok(Outcome::Duplicate));
    assert_eq!(s.encode(), before, "a duplicate event changed the state");
    assert_eq!(
        Stock::default().apply(&ev(0, 0, Change::Received { qty: 1 })),
        Ok(Outcome::Duplicate)
    );
}

#[test]
fn a_redelivered_event_in_the_log_is_skipped() {
    let events = log();
    let redelivered = vec![events[0], events[1], events[1], events[2], events[3]];
    assert_eq!(
        replay(&redelivered).map(|s| s.encode()),
        replay(&events).map(|s| s.encode())
    );
}

#[test]
fn a_duplicate_at_u64_max_does_not_overflow() {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&u64::MAX.to_be_bytes());
    let mut s = Stock::decode(&bytes).expect("32 bytes");
    let last = Event {
        version: u64::MAX,
        id: EventId(0),
        at_ms: 0,
        change: Change::Received { qty: 1 },
    };
    assert_eq!(s.apply(&last), Ok(Outcome::Duplicate));
}
