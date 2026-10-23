//! 11 intermediate: bool flags to an enum
//!
//! Think further, from the kata README:
//! 1. The business adds refunds: a shipped order can be refunded. List every place in `src/` that has to change. Which ones does the compiler point you to, and which do you have to find by reading?
//! 2. A client retries `cancel` after a timeout, but the first call had already worked. Should the second call return `Err(InvalidTransition)` or `Ok(())`? What does each choice mean for a client on a flaky network?
//! 3. A shipped order needs a tracking number, and a cancelled one needs a reason. Where does that data live, so that a cancelled order can't have a tracking number? What does that change about returning the state as a copy?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug, Clone, PartialEq, Eq)]
enum OrderState {
    New,
    Shipped { tracking: String },
    Cancelled { reason: String },
}

#[derive(Debug, PartialEq, Eq)]
enum Error {
    InvalidTransition,
}

impl OrderState {
    // Think further 2: a repeat cancel returns Ok, so a retry after a timeout is safe.
    fn cancel(self, reason: &str) -> Result<OrderState, Error> {
        match self {
            OrderState::New => Ok(OrderState::Cancelled { reason: reason.to_string() }),
            OrderState::Cancelled { .. } => Ok(self),
            OrderState::Shipped { .. } => Err(Error::InvalidTransition),
        }
    }
}

// In the wild: Poll is one enum, so there is no ready flag next to an Option to disagree with.
#[derive(Debug)]
enum Poll<T> {
    Ready(T),
    Pending,
}

fn main() {
    let order = OrderState::New;
    match order.cancel("customer asked") {
        Ok(cancelled) => {
            println!("{cancelled:?}");
            // Think further 2: the retry gets the same answer as the first call.
            println!("retry: {:?}", cancelled.clone().cancel("retry"));
        }
        Err(err) => println!("first cancel failed: {err:?}"),
    }

    // Think further 3: a shipped order carries its tracking number, and a cancelled one its reason.
    let shipped = OrderState::Shipped { tracking: "ZA123".to_string() };
    println!("cancel shipped: {:?}", shipped.clone().cancel("too late"));

    let p: Poll<u32> = Poll::Ready(7);
    println!("{p:?} {:?}", Poll::<u32>::Pending);
}
