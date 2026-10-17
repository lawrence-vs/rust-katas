use kata_lifetimes_03::reconnect;

#[test]
fn reconnect_uses_the_new_address() {
    assert_eq!(reconnect(), "192.168.1.1");
}
