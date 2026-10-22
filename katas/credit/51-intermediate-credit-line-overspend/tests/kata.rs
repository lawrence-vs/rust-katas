use kata_credit_51::{Cents, CreditError, CreditLine};
use loom::sync::Arc;
use loom::thread;

#[test]
fn reserve_within_the_limit_succeeds_and_beyond_it_fails() {
    loom::model(|| {
        let line = CreditLine::new(Cents(100));
        assert!(line.reserve(Cents(60)).is_ok());
        assert_eq!(line.available(), Cents(40));
        assert_eq!(
            line.reserve(Cents(50)).err(),
            Some(CreditError::Insufficient {
                available: Cents(40),
                requested: Cents(50)
            }),
            "a reserve past the limit is refused"
        );
    });
}

#[test]
fn a_cancelled_order_gives_its_credit_back() {
    loom::model(|| {
        let line = CreditLine::new(Cents(100));
        let r = line.reserve(Cents(60)).unwrap();
        line.cancel(r);
        assert_eq!(
            line.available(),
            Cents(100),
            "a cancelled order gives its credit back"
        );
    });
}

#[test]
fn a_partial_fill_releases_the_remainder() {
    loom::model(|| {
        let line = CreditLine::new(Cents(100));
        let r = line.reserve(Cents(60)).unwrap();
        assert_eq!(line.fill(r, Cents(25)), Ok(()));
        assert_eq!(
            line.available(),
            Cents(75),
            "a partial fill releases the unfilled remainder"
        );
    });
}

#[test]
fn an_overfill_is_rejected_and_releases_nothing() {
    loom::model(|| {
        let line = CreditLine::new(Cents(100));
        let r = line.reserve(Cents(60)).unwrap();
        assert_eq!(
            line.fill(r, Cents(61)),
            Err(CreditError::Overfill {
                reserved: Cents(60),
                filled: Cents(61)
            })
        );
        assert_eq!(
            line.available(),
            Cents(40),
            "an overfill is rejected and releases nothing"
        );
    });
}

#[test]
fn two_orders_cannot_overspend_the_line() {
    loom::model(|| {
        let line = Arc::new(CreditLine::new(Cents(100)));
        let other = Arc::clone(&line);
        let handle = thread::spawn(move || other.reserve(Cents(60)).is_ok());
        let mine = line.reserve(Cents(60)).is_ok();
        let theirs = handle.join().unwrap();
        assert!(!(mine && theirs), "two orders overspent the credit line");
        assert!(mine || theirs, "one of two orders fits the credit line");
        assert_eq!(line.available(), Cents(40));
    });
}

#[test]
fn a_cancel_racing_a_reserve_is_not_lost() {
    loom::model(|| {
        let line = Arc::new(CreditLine::new(Cents(100)));
        let r = line.reserve(Cents(60)).unwrap();
        let other = Arc::clone(&line);
        let handle = thread::spawn(move || other.cancel(r));
        assert!(line.reserve(Cents(30)).is_ok());
        handle.join().unwrap();
        assert_eq!(
            line.available(),
            Cents(70),
            "a cancel racing a reserve is not lost"
        );
    });
}

#[test]
fn a_limit_of_u64_max_does_not_overflow() {
    loom::model(|| {
        let line = CreditLine::new(Cents(u64::MAX));
        assert!(line.reserve(Cents(u64::MAX)).is_ok());
        assert_eq!(
            line.reserve(Cents(1)).err(),
            Some(CreditError::Insufficient {
                available: Cents(0),
                requested: Cents(1)
            }),
            "a full line refuses without overflowing"
        );
    });
}
