use kata_closures_37::process_each;

#[test]
fn accepts_fnmut_closure() {
    let mut sum = 0;
    process_each(&[1, 2, 3], |x| sum += x);
    assert_eq!(sum, 6);
}

#[test]
fn accepts_fn_closure() {
    let factor = 2;
    let mut sum = 0;
    process_each(&[1, 2, 3], |x| sum += x * factor);
    assert_eq!(sum, 12);
}

#[test]
fn accepts_fn_once_closure() {
    let data = vec![10, 20, 30];
    process_each(&[1, 2, 3], |_| drop(data.clone()));
    // If the bound were FnOnce, this would fail on the second call.
    // With FnMut/Fn, the closure can be called multiple times.
}
