use kata_ownership_05::Job;
#[test]
fn finish_hands_over_the_output() {
    let mut job = Job {
        output: Some(String::from("done")),
    };
    assert_eq!(job.finish(), Some(String::from("done")));
    assert_eq!(job.output, None);
}
