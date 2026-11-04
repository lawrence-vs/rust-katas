#![deny(unfulfilled_lint_expectations)]

/// CLUE: does this function ever go unused, given who calls it?
#[expect(dead_code, reason = "only the CLI calls this")]
fn legacy_key() -> &'static str {
    "k-legacy"
}

/// CLUE: what does the compiler say about an expectation that is never met?
pub fn key() -> &'static str {
    legacy_key()
}
