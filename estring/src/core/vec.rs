//! Contains the implementations to vec type
//!
//! **NOTE**: Require the enabling of the `vec` features
//!

use crate::core::EString;
use std::convert::TryFrom;
use std::fmt::Write;

/// Wrapper for ``Vec`` to split string by a separator (`SEP`).
///
/// **NOTE**: Required the enabling of the `vec` feature.
///
/// # Examples
///
/// ```rust
/// use estring::{SepVec, EString};
///
/// type CommaVec<T> = SepVec<T, ','>;
///
/// fn main() -> Result<(), estring::ParseError> {
///     let res = EString::from("1,2,3").parse::<CommaVec<u8>>()?;
///     assert_eq!(*res, vec![1, 2, 3]);
///
///     Ok(())
/// }
///
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct SepVec<T, const SEP: char>(pub Vec<T>);

impl<T, const SEP: char> std::ops::Deref for SepVec<T, SEP> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const SEP: char> From<Vec<T>> for SepVec<T, SEP> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T, const SEP: char> std::fmt::Display for SepVec<T, SEP>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().try_for_each(|(i, part)| {
            if i != 0 {
                f.write_char(SEP)?;
            }

            f.write_str(&part.to_string())
        })
    }
}

impl<T, const SEP: char> TryFrom<EString> for SepVec<T, SEP>
where
    T: TryFrom<EString> + std::fmt::Display,
{
    type Error = T::Error;

    fn try_from(value: EString) -> Result<Self, Self::Error> {
        let inner = value
            .split(SEP)
            .map(str::trim)
            .map(EString::from)
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMA: char = ',';
    const SEMI: char = ';';

    type CommaVec<T> = SepVec<T, COMMA>;
    type SemiVec<T> = SepVec<T, SEMI>;

    #[test]
    fn should_parse_into_vec() {
        let estr = EString::from("a,b,c,d,e");
        match estr.parse::<CommaVec<&str>>() {
            Ok(res) => assert_eq!(*res, vec!["a", "b", "c", "d", "e"]),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_trim_identations_before_parsing() {
        let input = "
a , b, c,
d,e";
        let estr = EString::from(input);
        match estr.parse::<CommaVec<&str>>() {
            Ok(res) => assert_eq!(*res, vec!["a", "b", "c", "d", "e"]),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_into_vector_of_vectors() {
        let estr = EString::from("a,b; c,d,e; f,g");
        match estr.parse::<SemiVec<CommaVec<&str>>>() {
            Ok(res) => assert_eq!(
                res,
                SemiVec::from(vec![
                    CommaVec::from(vec!["a", "b"]),
                    CommaVec::from(vec!["c", "d", "e"]),
                    CommaVec::from(vec!["f", "g"])
                ])
            ),
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "number")]
    mod numbers {
        use super::*;
        use crate::ParseError;

        #[test]
        fn should_parse_into_num_vec() {
            let estr = EString::from("1,2,3,4,5");
            match estr.parse::<CommaVec<i32>>() {
                Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_throw_parse_vec_error() {
            let estr = EString::from("1,2,3,4,5");
            match estr.parse::<SemiVec<i32>>() {
                Err(ParseError(orig)) => {
                    assert_eq!(orig, String::from("1,2,3,4,5"));
                }
                _ => unreachable!(),
            };
        }
    }
}
