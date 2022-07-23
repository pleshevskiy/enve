//! Contains the ``EString`` type, as well as the basic implementation of conversions to
//! string types
//!
#[cfg(any(feature = "number", feature = "bool"))]
pub mod prim;
#[cfg(any(feature = "number", feature = "bool"))]
pub use prim::*;

#[cfg(feature = "vec")]
pub mod vec;
#[cfg(feature = "vec")]
pub use vec::*;

use crate::ParseError;
use std::convert::{Infallible, TryFrom};

/// Wrapper under String type.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct EString(pub String);

impl EString {
    /// Parses inner string by type annotations and returns result.
    ///
    /// # Errors
    ///
    /// Will return `Err` if estring cannot parse inner fragment
    ///
    #[inline]
    pub fn parse<T: TryFrom<EString>>(self) -> Result<T, ParseError> {
        let orig = self.0.clone();
        <T as TryFrom<EString>>::try_from(self).map_err(|_| ParseError(orig))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deref_to_string() {
        let estr = EString::from("hello");
        assert_eq!(*estr, String::from("hello"));
    }

    #[test]
    fn should_parse_into_itself() {
        let estr = EString::from("hello");
        match estr.parse::<EString>() {
            Ok(res) => assert_eq!(res, EString::from("hello")),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_parse_into_string() {
        let estr = EString::from("hello");
        match estr.parse::<String>() {
            Ok(res) => assert_eq!(res, String::from("hello")),
            _ => unreachable!(),
        }
    }
}
