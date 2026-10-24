use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownAccount,
    InsufficientFunds,
    UnknownOrder,
}

struct Orders {
    next: u64,
    open: HashMap<OrderId, (AccountId, u64)>,
}

/// CLUE: Do `balances` and `orders` ever change apart from each other? What would one lock, or one owner, cost?
pub struct Exchange {
    balances: Mutex<HashMap<AccountId, u64>>,
    orders: Mutex<Orders>,
    hook: Box<dyn Fn() + Send + Sync>,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Default for Exchange {
    fn default() -> Self {
        Self::new()
    }
}

impl Exchange {
    pub fn new() -> Self {
        Self::with_hook(|| {})
    }

    /// `place` and `cancel` call `hook` exactly once each, while holding at least one of the exchange's locks.
    pub fn with_hook(hook: impl Fn() + Send + Sync + 'static) -> Self {
        Exchange {
            balances: Mutex::new(HashMap::new()),
            orders: Mutex::new(Orders {
                next: 1,
                open: HashMap::new(),
            }),
            hook: Box::new(hook),
        }
    }

    /// Creates the account if it is new. Overflow saturates at `u64::MAX`.
    pub fn deposit(&self, account: AccountId, amount: u64) {
        let mut balances = lock(&self.balances);
        let entry = balances.entry(account).or_insert(0);
        *entry = entry.saturating_add(amount);
    }

    /// Free balance; 0 if the account is unknown.
    pub fn balance(&self, account: AccountId) -> u64 {
        lock(&self.balances).get(&account).copied().unwrap_or(0)
    }

    /// Reserves `cost` from the account's free balance.
    ///
    /// CLUE: Write down which lock `place` takes first and which `cancel` takes first. What does each thread hold when it reaches the hook?
    pub fn place(&self, account: AccountId, cost: u64) -> Result<OrderId, Error> {
        let mut balances = lock(&self.balances);
        let free = balances.get_mut(&account).ok_or(Error::UnknownAccount)?;
        if *free < cost {
            return Err(Error::InsufficientFunds);
        }
        *free -= cost;
        (self.hook)();
        let mut orders = lock(&self.orders);
        let id = OrderId(orders.next);
        orders.next += 1;
        orders.open.insert(id, (account, cost));
        Ok(id)
    }

    /// Refunds the reservation of an open order.
    ///
    /// CLUE: A deadlock needs each thread to hold something the other one wants. Which of those conditions can you take away and still change both maps in one step?
    pub fn cancel(&self, order: OrderId) -> Result<(), Error> {
        let mut orders = lock(&self.orders);
        let (account, cost) = orders.open.remove(&order).ok_or(Error::UnknownOrder)?;
        (self.hook)();
        let mut balances = lock(&self.balances);
        let free = balances.entry(account).or_insert(0);
        *free = free.saturating_add(cost);
        Ok(())
    }

    pub fn open_orders(&self) -> usize {
        lock(&self.orders).open.len()
    }
}
