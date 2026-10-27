use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub id: u64,
    pub account: String,
    pub amount: i64,
}

#[derive(Default)]
pub struct Ledger {
    balances: BTreeMap<String, i64>,
}

impl Ledger {
    /// Apply one delivered event. The broker redelivers events, so a repeat must not move a balance.
    /// CLUE: Between two calls, what must the ledger remember to recognise an event it has already applied?
    pub fn apply(&mut self, event: &Event) {
        *self.balances.entry(event.account.clone()).or_insert(0) += event.amount;
    }

    pub fn balance(&self, account: &str) -> i64 {
        self.balances.get(account).copied().unwrap_or(0)
    }
}
