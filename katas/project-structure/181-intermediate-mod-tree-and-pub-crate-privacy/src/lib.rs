//! A shopping cart with a percentage discount.
//!
//! Another crate gets a `Percent` only through `Percent::new`, and reaches
//! nothing but `percent::{Percent, PercentError}` and `cart::Cart`. Each block
//! below is a separate crate and must not compile:
//!
//! ```compile_fail
//! use kata_project_structure_181::percent::Percent;
//! let _p = Percent { value: 150 };
//! ```
//!
//! ```compile_fail
//! use kata_project_structure_181::percent::Percent;
//! let mut p = Percent::new(20).unwrap();
//! p.value = 150;
//! ```
//!
//! ```compile_fail
//! use kata_project_structure_181::cart::Cart;
//! let cart = Cart::new();
//! let _n = cart.lines.len();
//! ```
//!
//! ```compile_fail
//! use kata_project_structure_181::money::share;
//! ```
//!
//! ```compile_fail
//! use kata_project_structure_181::cart::total::line_total;
//! ```

pub mod cart;
/// CLUE: Which crate calls `money::share`? Once nothing outside this crate can reach it, what does the `unreachable_pub` line in Cargo.toml ask a `pub fn` in here to say instead?
pub mod money;
pub mod percent;
