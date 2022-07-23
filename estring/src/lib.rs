//!
//! # ``EString``
//!
//! A simple way to parse a string using type annotations.
//!
//! This package was originally designed for [enve]
//!
//! [enve]: https://github.com/pleshevskiy/itconfig-rs
//!
//! ## Getting started
//!
//! ```rust
//! use estring::{SepVec, EString};
//!
//! type PlusVec<T> = SepVec<T, '+'>;
//! type MulVec<T> = SepVec<T, '*'>;
//!
//! fn main() -> Result<(), estring::ParseError> {
//!     let res = EString::from("10+5*2+3")
//!         .parse::<PlusVec<MulVec<f32>>>()?
//!         .iter()
//!         .map(|m| m.iter().product::<f32>())
//!         .sum::<f32>();
//!
//!     assert_eq!(res, 23.0);
//!     Ok(())
//! }
//! ```
//!
//! ## Installation
//!
//! **The MSRV is 1.51.0**
//!
//! Add `estring = { version = "0.1", features = ["vec", "number"] }` as a
//! dependency in `Cargo.toml`.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [package]
//! name = "my-crate"
//! version = "0.1.0"
//! authors = ["Me <user@rust-lang.org>"]
//!
//! [dependencies]
//! estring = { version = "0.1", features = ["vec", "number"] }
//! ```
//!
//! ## License
//!
//! **MIT**.
//!
//! See [LICENSE](./LICENSE) to see the full text.
//!
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![deny(missing_docs)]

pub mod core;
mod error;

pub use crate::core::*;
pub use crate::error::ParseError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
