#[cfg(feature = "vec")]
mod vec;
#[cfg(feature = "vec")]
pub use vec::{CommaVec, SemiVec};

pub use estring::core::*;
