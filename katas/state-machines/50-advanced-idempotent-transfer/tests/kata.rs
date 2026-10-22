use kata_state_machines_50::{
    Cents, Event, Rail, State, TimedOut, TransferError, TransferKey, Transfers,
};

#[derive(Default)]
struct FakeRail {
    sent: Vec<(TransferKey, Cents)>,
    timeouts: usize,
}

impl Rail for FakeRail {
    fn send(&mut self, key: TransferKey, amount: Cents) -> Result<(), TimedOut> {
        self.sent.push((key, amount)); // the money moves even when the answer is lost
        if self.timeouts > 0 {
            self.timeouts -= 1;
            Err(TimedOut)
        } else {
            Ok(())
        }
    }
}

const K1: TransferKey = TransferKey(1);
const K2: TransferKey = TransferKey(2);
const AMOUNT: Cents = Cents(250_000);

fn request(key: TransferKey) -> Event {
    Event::Requested {
        key,
        amount: AMOUNT,
    }
}

fn transfers(timeouts: usize) -> Transfers<FakeRail> {
    Transfers::new(FakeRail {
        sent: Vec::new(),
        timeouts,
    })
}

#[test]
fn a_new_transfer_is_sent_once() {
    let mut t = transfers(0);
    assert_eq!(
        t.apply(request(K1)),
        Ok(State::InFlight),
        "a new transfer is sent once"
    );
    assert_eq!(
        t.rail().sent,
        vec![(K1, AMOUNT)],
        "a new transfer is sent once"
    );
    assert_eq!(
        t.state(K1),
        Some(State::InFlight),
        "a new transfer is sent once"
    );
}

#[test]
fn a_retry_after_a_timeout_does_not_send_twice() {
    let mut t = transfers(1);
    assert_eq!(t.apply(request(K1)), Ok(State::Unknown));
    let second = t.apply(request(K1));
    assert_eq!(
        t.rail().sent.len(),
        1,
        "a retry after a timeout does not send twice"
    );
    assert_eq!(
        second,
        Ok(State::Unknown),
        "a retry after a timeout does not send twice"
    );
}

#[test]
fn a_retry_while_in_flight_does_not_send_twice() {
    let mut t = transfers(0);
    assert_eq!(t.apply(request(K1)), Ok(State::InFlight));
    let second = t.apply(request(K1));
    assert_eq!(
        t.rail().sent.len(),
        1,
        "a retry while in flight does not send twice"
    );
    assert_eq!(
        second,
        Ok(State::InFlight),
        "a retry while in flight does not send twice"
    );
}

#[test]
fn a_retry_after_settlement_returns_settled() {
    let mut t = transfers(0);
    assert_eq!(t.apply(request(K1)), Ok(State::InFlight));
    assert_eq!(t.apply(Event::Settled { key: K1 }), Ok(State::Settled));
    let again = t.apply(request(K1));
    assert_eq!(
        t.rail().sent.len(),
        1,
        "a retry after settlement returns settled"
    );
    assert_eq!(
        again,
        Ok(State::Settled),
        "a retry after settlement returns settled"
    );
}

#[test]
fn the_rail_settles_a_transfer_whose_send_timed_out() {
    let mut t = transfers(1);
    assert_eq!(t.apply(request(K1)), Ok(State::Unknown));
    assert_eq!(t.apply(Event::Settled { key: K1 }), Ok(State::Settled));
    assert_eq!(
        t.state(K1),
        Some(State::Settled),
        "the rail settles a transfer whose send timed out"
    );
}

#[test]
fn a_rejected_transfer_retried_stays_rejected() {
    let mut t = transfers(0);
    assert_eq!(t.apply(request(K1)), Ok(State::InFlight));
    assert_eq!(t.apply(Event::Rejected { key: K1 }), Ok(State::Rejected));
    let again = t.apply(request(K1));
    assert_eq!(
        t.rail().sent.len(),
        1,
        "a rejected transfer retried stays rejected"
    );
    assert_eq!(
        again,
        Ok(State::Rejected),
        "a rejected transfer retried stays rejected"
    );
}

#[test]
fn a_key_reused_for_a_different_amount_is_refused() {
    let mut t = transfers(0);
    assert_eq!(t.apply(request(K1)), Ok(State::InFlight));
    let other = t.apply(Event::Requested {
        key: K1,
        amount: Cents(1),
    });
    assert_eq!(
        t.rail().sent.len(),
        1,
        "a key reused for a different amount is refused"
    );
    assert_eq!(
        other,
        Err(TransferError::KeyReused(K1)),
        "a key reused for a different amount is refused"
    );
    assert_eq!(
        t.state(K1),
        Some(State::InFlight),
        "a key reused for a different amount is refused"
    );
}

#[test]
fn an_outcome_for_an_unknown_transfer_is_an_error() {
    let mut t = transfers(0);
    assert_eq!(
        t.apply(Event::Settled { key: K1 }),
        Err(TransferError::UnknownTransfer(K1)),
        "an outcome for an unknown transfer is an error"
    );
    assert_eq!(
        t.apply(Event::Rejected { key: K2 }),
        Err(TransferError::UnknownTransfer(K2)),
        "an outcome for an unknown transfer is an error"
    );
    assert_eq!(
        t.state(K1),
        None,
        "an outcome for an unknown transfer is an error"
    );
}

#[test]
fn a_final_state_is_not_overwritten() {
    let mut t = transfers(0);
    assert_eq!(t.apply(request(K1)), Ok(State::InFlight));
    assert_eq!(t.apply(Event::Settled { key: K1 }), Ok(State::Settled));
    assert_eq!(
        t.apply(Event::Rejected { key: K1 }),
        Err(TransferError::Conflict {
            key: K1,
            state: State::Settled
        }),
        "a final state is not overwritten"
    );
    assert_eq!(
        t.state(K1),
        Some(State::Settled),
        "a final state is not overwritten"
    );

    assert_eq!(t.apply(request(K2)), Ok(State::InFlight));
    assert_eq!(t.apply(Event::Rejected { key: K2 }), Ok(State::Rejected));
    assert_eq!(
        t.apply(Event::Settled { key: K2 }),
        Err(TransferError::Conflict {
            key: K2,
            state: State::Rejected
        }),
        "a final state is not overwritten"
    );
    assert_eq!(
        t.state(K2),
        Some(State::Rejected),
        "a final state is not overwritten"
    );
}

#[test]
fn replaying_the_whole_log_is_a_no_op() {
    let log = [
        request(K1),
        Event::Settled { key: K1 },
        request(K2),
        Event::Rejected { key: K2 },
    ];
    let mut t = transfers(0);
    for event in log {
        t.apply(event).ok();
    }
    let replayed: Vec<_> = log.iter().map(|event| t.apply(*event)).collect();
    assert_eq!(t.rail().sent.len(), 2, "replaying the whole log is a no-op");
    assert_eq!(
        replayed,
        vec![
            Ok(State::Settled),
            Ok(State::Settled),
            Ok(State::Rejected),
            Ok(State::Rejected)
        ],
        "replaying the whole log is a no-op"
    );
    assert_eq!(
        t.state(K1),
        Some(State::Settled),
        "replaying the whole log is a no-op"
    );
    assert_eq!(
        t.state(K2),
        Some(State::Rejected),
        "replaying the whole log is a no-op"
    );
}
