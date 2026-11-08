use kata_ownership_220::Session;

#[test]
fn ack_front_acks_and_records() {
    let mut s = Session::new(&[7, 8]);
    assert_eq!(s.ack_front(), Some(7));
    assert!(s.queue[0].acked);
    assert!(!s.queue[1].acked);
    assert_eq!(s.sent, 1);
}

#[test]
fn ack_front_on_empty_is_none() {
    let mut s = Session::new(&[]);
    assert_eq!(s.ack_front(), None);
    assert_eq!(s.sent, 0);
}

#[test]
fn second_ack_front_acks_next() {
    let mut s = Session::new(&[7, 8]);
    s.ack_front();
    assert_eq!(s.ack_front(), Some(8));
    assert!(s.queue[1].acked);
    assert_eq!(s.sent, 2);
}

#[test]
fn next_id_counts_up() {
    let mut s = Session::new(&[]);
    assert_eq!([s.next_id(), s.next_id(), s.next_id()], [1, 2, 3]);
}
