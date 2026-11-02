use kata_testing_225::wait_until;

#[test]
#[should_panic(expected = "condition never met")]
fn panics_when_the_condition_never_holds() {
    wait_until(|| false, 5);
}

#[test]
fn returns_once_the_condition_holds() {
    let mut n = 0;
    wait_until(
        || {
            n += 1;
            n == 3
        },
        10,
    );
    assert_eq!(n, 3);
}
