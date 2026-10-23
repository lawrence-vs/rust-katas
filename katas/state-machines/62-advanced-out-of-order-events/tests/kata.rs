use kata_state_machines_62::{
    ConsumerError, Event, Message, OrderId, Orders, Outcome, State, TransitionError,
};
use proptest::prelude::*;

const O1: OrderId = OrderId(1);
const O2: OrderId = OrderId(2);

fn msg(order: OrderId, version: u64, event: Event) -> Message {
    Message {
        order,
        version,
        event,
    }
}

#[test]
fn in_order_events_apply() {
    let mut orders = Orders::new(4);
    assert_eq!(
        orders.on_message(msg(O1, 1, Event::Accepted)),
        Ok(Outcome::Applied { state: State::Open })
    );
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::PartiallyFilled)),
        Ok(Outcome::Applied {
            state: State::PartiallyFilled
        })
    );
    assert_eq!(
        orders.on_message(msg(O1, 3, Event::Filled)),
        Ok(Outcome::Applied {
            state: State::Filled
        })
    );
    assert_eq!(orders.state(O1), Some(State::Filled));
    assert_eq!(orders.version(O1), Some(3));
}

#[test]
fn the_transition_table_rejects_a_fill_on_a_new_order() {
    assert_eq!(
        State::New.apply(Event::Filled),
        Err(TransitionError::Invalid {
            from: State::New,
            event: Event::Filled
        })
    );
}

#[test]
fn a_fill_before_its_accept_is_buffered() {
    let mut orders = Orders::new(4);
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::Filled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(orders.state(O1), Some(State::New));
    assert_eq!(orders.buffered(O1), 1);
    assert_eq!(
        orders.on_message(msg(O1, 1, Event::Accepted)),
        Ok(Outcome::Applied {
            state: State::Filled
        })
    );
    assert_eq!(orders.buffered(O1), 0);
    assert_eq!(orders.version(O1), Some(2));
}

#[test]
fn a_duplicate_is_a_no_op() {
    let mut orders = Orders::new(4);
    orders.on_message(msg(O1, 1, Event::Accepted)).ok();
    assert_eq!(
        orders.on_message(msg(O1, 1, Event::Accepted)),
        Ok(Outcome::Ignored)
    );
    assert_eq!(orders.state(O1), Some(State::Open));
}

#[test]
fn a_stale_event_is_ignored() {
    let mut orders = Orders::new(4);
    orders.on_message(msg(O1, 1, Event::Accepted)).ok();
    orders.on_message(msg(O1, 2, Event::PartiallyFilled)).ok();
    orders.on_message(msg(O1, 3, Event::Cancelled)).ok();
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::PartiallyFilled)),
        Ok(Outcome::Ignored)
    );
    assert_eq!(orders.state(O1), Some(State::Cancelled));
}

#[test]
fn a_duplicate_while_buffered_is_held_once() {
    let mut orders = Orders::new(4);
    orders.on_message(msg(O1, 1, Event::Accepted)).ok();
    assert_eq!(
        orders.on_message(msg(O1, 3, Event::Filled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(
        orders.on_message(msg(O1, 3, Event::Filled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(orders.buffered(O1), 1);
}

#[test]
fn a_gap_longer_than_the_buffer_is_an_error() {
    let mut orders = Orders::new(2);
    orders.on_message(msg(O1, 1, Event::Accepted)).ok();
    assert_eq!(
        orders.on_message(msg(O1, 3, Event::PartiallyFilled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(
        orders.on_message(msg(O1, 4, Event::PartiallyFilled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(
        orders.on_message(msg(O1, 5, Event::Filled)),
        Err(ConsumerError::GapTooLong {
            order: O1,
            expected: 2,
            buffered: 2
        })
    );
    assert_eq!(orders.state(O1), Some(State::Open));
    assert_eq!(orders.buffered(O1), 2);
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::PartiallyFilled)),
        Ok(Outcome::Applied {
            state: State::PartiallyFilled
        })
    );
    assert_eq!(orders.version(O1), Some(4));
    assert_eq!(orders.buffered(O1), 0);
}

#[test]
fn an_invalid_transition_is_an_error_not_a_panic() {
    let mut orders = Orders::new(4);
    orders.on_message(msg(O1, 1, Event::Accepted)).ok();
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::Accepted)),
        Err(ConsumerError::Transition {
            order: O1,
            version: 2,
            source: TransitionError::Invalid {
                from: State::Open,
                event: Event::Accepted
            }
        })
    );
    assert_eq!(orders.state(O1), Some(State::Open));
    assert_eq!(orders.version(O1), Some(1));
}

#[test]
fn orders_are_independent() {
    let mut orders = Orders::new(4);
    assert_eq!(
        orders.on_message(msg(O1, 2, Event::Filled)),
        Ok(Outcome::Buffered)
    );
    assert_eq!(
        orders.on_message(msg(O2, 1, Event::Accepted)),
        Ok(Outcome::Applied { state: State::Open })
    );
    assert_eq!(orders.buffered(O1), 1);
}

fn script() -> impl Strategy<Value = Vec<Event>> {
    prop_oneof![
        Just(vec![Event::Rejected]),
        (
            0usize..=3,
            prop_oneof![Just(Event::Filled), Just(Event::Cancelled)]
        )
            .prop_map(|(n, last)| {
                let mut events = vec![Event::Accepted];
                events.extend(std::iter::repeat_n(Event::PartiallyFilled, n));
                events.push(last);
                events
            }),
    ]
}

fn arrivals() -> impl Strategy<Value = (Vec<Event>, Vec<Message>)> {
    script().prop_flat_map(|events| {
        let n = events.len();
        prop::collection::vec(1usize..=3, n).prop_flat_map(move |copies| {
            let mut all = Vec::new();
            for ((version, event), count) in (1u64..).zip(events.iter().copied()).zip(copies) {
                all.extend(std::iter::repeat_n(msg(O1, version, event), count));
            }
            let events = events.clone();
            Just(all)
                .prop_shuffle()
                .prop_map(move |all| (events.clone(), all))
        })
    })
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]
    #[test]
    fn any_arrival_order_with_duplicates_ends_in_the_same_state(
        (events, arrivals) in arrivals(),
    ) {
        let mut orders = Orders::new(16);
        for m in arrivals {
            let result = orders.on_message(m);
            prop_assert!(result.is_ok(), "message {:?} failed: {:?}", m, result);
        }
        let expected = match events.last() {
            Some(Event::Rejected) => State::Rejected,
            Some(Event::Filled) => State::Filled,
            Some(Event::Cancelled) => State::Cancelled,
            other => return Err(TestCaseError::fail(format!("bad script end {other:?}"))),
        };
        prop_assert_eq!(orders.state(O1), Some(expected));
        prop_assert_eq!(orders.buffered(O1), 0);
        prop_assert_eq!(orders.version(O1), Some(events.len() as u64));
    }
}
