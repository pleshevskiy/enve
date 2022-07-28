//! # enve
//!
//! `enve` helps you work with environment variables and convert it to **any type**
//! using only **type annotations**.
//!
//! All standard environment variable types are included, but `enve` under the hood
//! uses [estring](https://github.com/pleshevskiy/estring), so you can easily create
//! your own type.
//!
//! ## Usage
//!
//! Basic
//!
//! ```rust
//! fn main() -> Result<(), enve::Error> {
//!     enve::sset("E", "10");
//!
//!     let res: f32 = enve::get("E")?;
//!
//!     println!("result: {}", res);
//!
//!     Ok(())
//! }
//! ```
//!
//! You can use predefined structs like `SepVec` if you enable `structs` feature.
//!
//! Note: You can use custom types as annotations! Just implement `ParseFragment`.
//!
//! ```rust
//! use enve::SepVec;
//!
//! type PlusVec<T> = SepVec<T, '+'>;
//! type MulVec<T> = SepVec<T, '*'>;
//!
//! fn main() -> Result<(), enve::Error> {
//!     enve::sset("E", "10+5*2+3");
//!
//!     let res = enve::get::<PlusVec<MulVec<f32>>>("E")?
//!         .iter()
//!         .map(|m| m.iter().product::<f32>())
//!         .sum::<f32>();
//!
//!     assert_eq!(res, 23.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! You can also use predefined aggregators if you enable `aggs` feature.
//!
//! ```rust
//! use enve::{SepVec, Product, Sum, estring::Aggregate};
//!
//! type PlusVec<T> = SepVec<T, '+'>;
//! type MulVec<T> = SepVec<T, '*'>;
//!
//! fn main() -> Result<(), enve::Error> {
//!     enve::sset("E", "10+5*2+3");
//!
//!     let res = enve::get::<Sum<PlusVec<Product<MulVec<f32>>>>>("E")?.agg();
//!
//!     assert_eq!(res, 23.0);
//!
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! Look at the [examples] to see the power!
//!
//! [examples]: https://github.com/pleshevskiy/enve/tree/main/examples
//!

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

pub use estring;
