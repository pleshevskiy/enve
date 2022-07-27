#[cfg(feature = "structs")]
mod structs;
#[cfg(feature = "structs")]
pub use structs::{CommaVec, SemiVec};

pub use estring::core::EString;
pub use estring::low::*;
pub use estring::structs::*;
