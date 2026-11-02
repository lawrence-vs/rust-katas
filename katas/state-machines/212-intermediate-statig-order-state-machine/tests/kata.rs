use kata_state_machines_212::{Event, Order, State};
use statig::prelude::*;

fn order_after(events: &[Event]) -> StateMachine<Order> {
    let mut order = Order::default().state_machine();
    for event in events {
        order.handle(event);
    }
    order
}

#[test]
fn a_paid_order_ships() {
    let order = order_after(&[Event::Pay, Event::Ship]);
    assert!(matches!(order.state(), State::Shipped { .. }));
}

#[test]
fn a_created_order_cannot_ship() {
    let order = order_after(&[Event::Ship]);
    assert!(matches!(order.state(), State::Created { .. }));
}

#[test]
fn a_cancelled_order_stays_cancelled() {
    let order = order_after(&[Event::Cancel, Event::Pay, Event::Ship]);
    assert!(matches!(order.state(), State::Cancelled { .. }));
}

#[test]
fn shipped_order_refuses_cancel() {
    let order = order_after(&[Event::Pay, Event::Ship, Event::Cancel]);
    assert!(matches!(order.state(), State::Shipped { .. }));
}
