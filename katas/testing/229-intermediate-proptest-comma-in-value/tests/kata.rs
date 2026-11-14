use kata_testing_229::{decode, encode};
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trips(pairs in proptest::collection::vec(("[a-z]{1,4}", "[a-z,]{0,6}"), 0..5)) {
        prop_assert_eq!(decode(&encode(&pairs)), pairs);
    }
}

#[test]
fn encodes_one_pair() {
    assert_eq!(encode(&[("a".into(), "1".into())]), "a=1");
}
