use kata_testing_53::{Balance, BalanceError};
use proptest::prelude::*;

#[test]
fn try_from_rejects_a_negative_balance() {
    assert_eq!(
        Balance::try_from(-1),
        Err(BalanceError::Negative(-1)),
        "a negative balance was built"
    );
}

#[test]
fn try_from_accepts_zero_and_positive() {
    assert_eq!(Balance::try_from(0), Ok(Balance::ZERO));
    assert_eq!(
        i128::from(Balance::try_from(i64::MAX).unwrap()),
        i128::from(i64::MAX)
    );
}

#[test]
fn a_withdrawal_past_zero_is_refused() {
    assert_eq!(
        Balance::try_from(50).unwrap().withdraw(60),
        Err(BalanceError::Insufficient {
            balance: Balance::try_from(50).unwrap(),
            requested: 60
        }),
        "a withdrawal past zero was accepted"
    );
}

#[test]
fn a_withdrawal_of_the_whole_balance_is_allowed() {
    assert_eq!(
        Balance::try_from(50).unwrap().withdraw(50),
        Ok(Balance::ZERO)
    );
}

#[test]
fn a_huge_deposit_is_an_error_not_a_wrap() {
    assert_eq!(
        Balance::try_from(1).unwrap().deposit(u64::MAX),
        Err(BalanceError::Overflow),
        "a deposit wrapped instead of failing"
    );
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Deposit(u64),
    Withdraw(u64),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0u64..=1_000).prop_map(Op::Deposit),
        (0u64..=1_000).prop_map(Op::Withdraw),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]
    #[test]
    fn no_sequence_of_operations_makes_the_balance_negative(
        start in 0i64..=1_000,
        ops in prop::collection::vec(op(), 0..50),
    ) {
        let mut balance = Balance::try_from(start).unwrap();
        let mut model = i128::from(start);
        for op in ops {
            let next = match op {
                Op::Deposit(a) => balance.deposit(a),
                Op::Withdraw(a) => balance.withdraw(a),
            };
            match (op, next) {
                (_, Ok(b)) => {
                    model += match op {
                        Op::Deposit(a) => i128::from(a),
                        Op::Withdraw(a) => -i128::from(a),
                    };
                    balance = b;
                }
                (Op::Withdraw(a), Err(_)) => {
                    prop_assert!(i128::from(a) > model, "a valid withdrawal was refused")
                }
                (Op::Deposit(_), Err(_)) => prop_assert!(false, "a small deposit was refused"),
            }
            prop_assert!(i128::from(balance) >= 0, "balance went negative: {}", i128::from(balance));
            prop_assert_eq!(i128::from(balance), model, "balance drifted from the accepted operations");
        }
    }
}
