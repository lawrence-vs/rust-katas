use kata_events_207::{Event, Ledger};

fn event(id: u64, amount: i64) -> Event {
    Event {
        id,
        account: "alice".to_string(),
        amount,
    }
}

#[test]
fn redelivered_event_applies_once() {
    let mut ledger = Ledger::default();
    let deposit = event(7, 100);
    ledger.apply(&deposit);
    ledger.apply(&deposit);
    assert_eq!(ledger.balance("alice"), 100);
}

#[test]
fn distinct_events_both_apply() {
    let mut ledger = Ledger::default();
    ledger.apply(&event(7, 100));
    ledger.apply(&event(8, 50));
    assert_eq!(ledger.balance("alice"), 150);
}
