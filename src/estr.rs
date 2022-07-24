#[cfg(feature = "vec")]
pub mod vec;
#[cfg(feature = "vec")]
pub use vec::{CommaVec, SemiVec};

pub use estring::core::*;
