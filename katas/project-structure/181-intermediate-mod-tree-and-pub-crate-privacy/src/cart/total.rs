use super::Line;

/// CLUE: Which modules call this? Can its visibility name exactly those and nothing wider?
pub fn line_total(line: &Line) -> Option<u64> {
    line.unit_cents.checked_mul(u64::from(line.qty))
}
