/// `percent` per cent of `cents`, rounded down.
pub fn share(cents: u64, percent: u8) -> u64 {
    let wide = u128::from(cents) * u128::from(percent) / 100;
    u64::try_from(wide).unwrap_or(u64::MAX)
}
