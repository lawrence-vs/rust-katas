//! A transfer fee with one threshold.

/// From this many cents up, a transfer pays the percentage fee.
pub const LARGE_TRANSFER: u64 = 100_000;
/// The flat fee in cents for a transfer below `LARGE_TRANSFER`.
pub const SMALL_FEE: u64 = 150;
/// A large transfer pays `amount / LARGE_FEE_DIVISOR` cents (0.2%, rounded down).
pub const LARGE_FEE_DIVISOR: u64 = 500;

/// The fee in cents for a transfer of `amount` cents.
/// CLUE: Every case below passes whether this says `>=` or `>`. On which inputs do those two operators give different answers?
pub fn transfer_fee(amount: u64) -> u64 {
    if amount >= LARGE_TRANSFER {
        amount / LARGE_FEE_DIVISOR
    } else {
        SMALL_FEE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (amount in cents, expected fee in cents). Add rows here; change nothing else.
    /// CLUE: Both branches already run. What does running a line prove about the comparison on it?
    const CASES: &[(u64, u64)] = &[
        (0, 150),
        (50_000, 150),
        (99_999, 150),
        (250_000, 500),
        (1_000_000, 2_000),
    ];

    #[test]
    fn every_case_passes() {
        for &(amount, want) in CASES {
            assert_eq!(transfer_fee(amount), want, "transfer_fee({amount})");
        }
    }

    /// A hand-written copy of `transfer_fee` with `>=` changed to `>`. Don't edit.
    /// CLUE: If no case fails on this function, what does that say about the cases?
    fn transfer_fee_mutant(amount: u64) -> u64 {
        if amount > LARGE_TRANSFER {
            amount / LARGE_FEE_DIVISOR
        } else {
            SMALL_FEE
        }
    }

    #[test]
    fn the_cases_kill_the_ge_mutant() {
        let killed = CASES
            .iter()
            .any(|&(amount, want)| transfer_fee_mutant(amount) != want);
        assert!(killed, "mutant survived: no case tells >= from >");
    }
}
