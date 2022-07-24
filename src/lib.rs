//! # enve
//!
//! `enve` helps you work with environment variables and convert it to **any type**
//! using only **type annotations**.
//!
//! Look at the [examples](https://github.com/pleshevskiy/enve/tree/main/examples)
//! to see the power!
//!
//! All standard environment variable types are included, but `enve` under the hood
//! uses [estring](https://github.com/pleshevskiy/estring), so you can easily create
//! your own type.
//!
//! ## Getting started
//!
//! ```rust
//! use enve::SepVec;
//!
//! type MinusVec<T> = SepVec<T, '-'>;
//! type PlusVec<T> = SepVec<T, '+'>;
//! type MulVec<T> = SepVec<T, '*'>;
//!
//! fn main() -> Result<(), enve::Error> {
//!     enve::sset("E", "10+5*2-3");
//!
//!     let res: f32 = enve::get::<PlusVec<MinusVec<MulVec<f32>>>>("E")
//!         .unwrap()
//!         .iter()
//!         .map(|p| {
//!             p.iter()
//!                 .map(|m| m.iter().product::<f32>())
//!                 .reduce(|acc, v| acc - v)
//!                 .unwrap_or_default()
//!         })
//!         .sum::<f32>();
//!
//!     println!("result: {}", res);
//!
//!     Ok(())
//! }
//! ```

// Rustc lints.
#![forbid(unsafe_code)]
#![forbid(non_ascii_idents)]
#![deny(
    missing_debug_implementations,
    missing_docs,
    unstable_features,
    unused_imports,
    unused_qualifications
)]
// Clippy lints
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/////////////////////////////////////////////////////////////////////////////

mod core;
mod error;
mod estr;

pub use crate::core::{get, get_or_set_default, sget, sset};
pub use error::Error;
pub use estr::*;
