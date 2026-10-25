use kata_testing_162::{LARGE_FEE_DIVISOR, LARGE_TRANSFER, SMALL_FEE, transfer_fee};

#[test]
fn small_and_large_transfers_use_their_own_fee() {
    assert_eq!(transfer_fee(1), SMALL_FEE);
    assert_eq!(
        transfer_fee(LARGE_TRANSFER * 3),
        LARGE_TRANSFER * 3 / LARGE_FEE_DIVISOR
    );
}
