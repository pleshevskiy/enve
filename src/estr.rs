#[cfg(feature = "structs")]
mod structs;
#[cfg(feature = "structs")]
pub use structs::{CommaVec, SemiVec};

pub use estring::core::EString;

#[cfg(feature = "aggs")]
pub use estring::agg::*;
#[cfg(feature = "low-level")]
pub use estring::low::*;
#[cfg(feature = "structs")]
pub use estring::structs::*;
