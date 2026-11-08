/// CLUE: the snapshot shows dollars with two decimals; what unit is `price_cents` in?
pub fn receipt(symbol: &str, qty: u32, price_cents: u64) -> String {
    let total = u64::from(qty) * price_cents;
    format!("symbol: {symbol}\nqty: {qty}\ntotal: {total}")
}
