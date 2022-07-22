#[cfg(any(feature = "number", feature = "bool"))]
pub mod prim;
#[cfg(any(feature = "number", feature = "bool"))]
pub use prim::*;

#[cfg(feature = "vec")]
pub mod vec;
#[cfg(feature = "vec")]
pub use vec::*;

use std::convert::{Infallible, TryFrom};

/// Wrapper under String type.
///
/// When we read the environment variable, we automatically convert the value
/// to EnvString and then convert it to your expected type.
///
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct EString(String);

impl<T> From<T> for EString
where
    T: std::fmt::Display,
{
    #[inline]
    fn from(val: T) -> Self {
        Self(val.to_string())
    }
}

impl std::ops::Deref for EString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<EString> for String {
    type Error = Infallible;

    #[inline]
    fn try_from(s: EString) -> Result<Self, Self::Error> {
        Ok(s.0)
    }
}

impl TryFrom<EString> for &'static str {
    type Error = Infallible;

    #[inline]
    fn try_from(s: EString) -> Result<Self, Self::Error> {
        Ok(Box::leak(s.0.into_boxed_str()))
    }
}
