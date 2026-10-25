//! Splits a small arithmetic expression into tokens.
//!
//! Callers use `Token`, `Error`, `tokenize` and `prelude` from the crate root.
//! The module tree below is an implementation detail:
//!
//! ```compile_fail
//! use kata_project_structure_182::internal::parser::tokens::Token;
//! ```
//!
//! and the lexer's helper is not part of the API:
//!
//! ```compile_fail
//! use kata_project_structure_182::operator_token;
//! ```

/// CLUE: A doc test is another crate. Which paths can another crate name today, and which of them would break if `parser` moved?
/// CLUE: `pub use` gives an item a second path. Can that path stay the same while the item's real home moves?
/// CLUE: A glob re-export exports everything public in a module, helpers included.
pub mod internal;
