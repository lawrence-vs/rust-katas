use kata_closures_37::process_each;

#[test]
fn a_closure_may_mutate_captured_state() {
    let mut sum = 0;
    process_each(&[1, 2, 3], |x| sum += x);
    assert_eq!(sum, 6);
}

#[test]
fn a_closure_may_read_one_capture_and_mutate_another() {
    let factor = 2;
    let mut sum = 0;
    process_each(&[1, 2, 3], |x| sum += x * factor);
    assert_eq!(sum, 12);
}
