// Rustc lints.
#![deny(unused_imports)]

/////////////////////////////////////////////////////////////////////////////

mod getenv;
pub mod envstr;

pub use self::getenv::*;

pub mod prelude {
    pub use crate::envstr::*;
}


#[cfg(feature = "macro")]
#[allow(unused_imports)]
#[macro_use]
mod r#macro;
#[cfg(feature = "macro")]
#[doc(hidden)]
pub use r#macro::*;