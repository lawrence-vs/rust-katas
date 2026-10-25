//! Splits a small arithmetic expression into tokens.
//!
//! Callers use `Token`, `Error`, `tokenize` and `prelude` from the crate root.
//! The module tree below is an implementation detail:
//!
//! ```compile_fail
//! use kata_project_structure_182_refactored::syntax::token::Token;
//! ```
//!
//! and the helper that picks operator tokens is not part of the API:
//!
//! ```compile_fail
//! use kata_project_structure_182_refactored::syntax::token::operator_token;
//! ```

/// CLUE: This crate's internal names differ (`ScanError`, `scan`). Which keyword gives a re-export a different name?
pub mod syntax;
