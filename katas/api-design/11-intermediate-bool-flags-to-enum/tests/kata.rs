use kata_api_design_11::{InvalidTransition, Order, OrderState};

#[test]
fn happy_path() {
    let mut order = Order::new();
    assert_eq!(order.state(), OrderState::Created);
    order.pay().unwrap();
    order.ship().unwrap();
    assert_eq!(order.state(), OrderState::Shipped);
}

#[test]
fn cannot_ship_unpaid() {
    let mut order = Order::new();
    assert_eq!(order.ship(), Err(InvalidTransition));
    assert_eq!(order.state(), OrderState::Created);
}

#[test]
fn cannot_cancel_after_shipping() {
    let mut order = Order::new();
    order.pay().unwrap();
    order.ship().unwrap();
    assert_eq!(order.cancel(), Err(InvalidTransition));
    assert_eq!(order.state(), OrderState::Shipped);
}

#[test]
fn cannot_pay_a_cancelled_order() {
    let mut order = Order::new();
    order.cancel().unwrap();
    assert_eq!(order.pay(), Err(InvalidTransition));
    assert_eq!(order.state(), OrderState::Cancelled);
}
