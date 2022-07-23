//! Contains the implementations to primitive types (number, boolean)
//!
//! **NOTE**: Require the enabling of the same-name features
//!

#[cfg(feature = "bool")]
mod bool;
#[cfg(feature = "bool")]
pub use self::bool::*;

#[cfg(feature = "number")]
mod number;
#[cfg(feature = "number")]
pub use self::number::*;
