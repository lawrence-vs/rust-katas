use kata_testing_161::{Wallet, WalletError};
use proptest::prelude::*;
use proptest::test_runner::RngSeed;

#[derive(Debug, Clone, Copy)]
enum Op {
    Deposit(u64),
    Reserve(u64),
    Release(u64),
    Spend(u64),
}

fn apply(wallet: &mut Wallet, op: Op) -> Result<(), WalletError> {
    match op {
        Op::Deposit(a) => wallet.deposit(a),
        Op::Reserve(a) => wallet.reserve(a),
        Op::Release(a) => wallet.release(a),
        Op::Spend(a) => wallet.spend(a),
    }
}

struct Case {
    name: &'static str,
    ops: &'static [Op],
    last: Result<(), WalletError>,
    free: u64,
    reserved: u64,
    total: u64,
}

const CASES: &[Case] = &[
    Case {
        name: "deposit",
        ops: &[Op::Deposit(100)],
        last: Ok(()),
        free: 100,
        reserved: 0,
        total: 100,
    },
    Case {
        name: "reserve",
        ops: &[Op::Deposit(100), Op::Reserve(30)],
        last: Ok(()),
        free: 70,
        reserved: 30,
        total: 100,
    },
    Case {
        name: "reserve then release",
        ops: &[Op::Deposit(100), Op::Reserve(30), Op::Release(30)],
        last: Ok(()),
        free: 100,
        reserved: 0,
        total: 100,
    },
    Case {
        name: "spend",
        ops: &[Op::Deposit(100), Op::Spend(40)],
        last: Ok(()),
        free: 60,
        reserved: 0,
        total: 60,
    },
    Case {
        name: "reserve more than free",
        ops: &[Op::Deposit(50), Op::Reserve(80)],
        last: Err(WalletError::Insufficient {
            needed: 80,
            available: 50,
        }),
        free: 50,
        reserved: 0,
        total: 50,
    },
    Case {
        name: "spend more than the balance",
        ops: &[Op::Deposit(50), Op::Spend(80)],
        last: Err(WalletError::Insufficient {
            needed: 80,
            available: 50,
        }),
        free: 50,
        reserved: 0,
        total: 50,
    },
    Case {
        name: "deposit that overflows",
        ops: &[Op::Deposit(u64::MAX), Op::Deposit(1)],
        last: Err(WalletError::Overflow),
        free: u64::MAX,
        reserved: 0,
        total: u64::MAX,
    },
];

#[test]
fn hand_picked_cases() {
    for case in CASES {
        let mut wallet = Wallet::new();
        let (last, setup) = case.ops.split_last().expect("a case has at least one op");
        for op in setup {
            assert_eq!(
                apply(&mut wallet, *op),
                Ok(()),
                "{}: setup {:?} failed",
                case.name,
                op
            );
        }
        assert_eq!(
            apply(&mut wallet, *last),
            case.last,
            "{}: last op",
            case.name
        );
        assert_eq!(
            (wallet.free(), wallet.reserved(), wallet.total()),
            (case.free, case.reserved, case.total),
            "{}: free, reserved, total",
            case.name
        );
    }
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0u64..=100).prop_map(Op::Deposit),
        (0u64..=100).prop_map(Op::Reserve),
        (0u64..=100).prop_map(Op::Release),
        (0u64..=100).prop_map(Op::Spend),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig {
        rng_seed: RngSeed::Fixed(161),
        failure_persistence: None,
        ..ProptestConfig::default()
    })]
    #[test]
    fn total_is_free_plus_reserved_after_any_sequence(ops in prop::collection::vec(op(), 0..40)) {
        let mut wallet = Wallet::new();
        for op in ops {
            let before = wallet;
            let result = apply(&mut wallet, op);
            prop_assert_eq!(
                u128::from(wallet.total()),
                u128::from(wallet.free()) + u128::from(wallet.reserved()),
                "total != free + reserved after {:?} on {:?}", op, before
            );
            let allowed = match op {
                Op::Deposit(_) => true,
                Op::Reserve(a) | Op::Spend(a) => a <= before.free(),
                Op::Release(a) => a <= before.reserved(),
            };
            prop_assert_eq!(result.is_ok(), allowed, "{:?} on {:?} returned {:?}", op, before, result);
            if result.is_err() {
                prop_assert_eq!(wallet, before, "a refused {:?} changed the wallet", op);
            }
        }
    }
}
