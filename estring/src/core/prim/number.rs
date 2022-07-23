use crate::core::EString;
use std::convert::TryFrom;

#[doc(hidden)]
macro_rules! from_env_string_numbers_impl {
    ($($ty:ty),+$(,)?) => {
        $(
            #[cfg(feature = "number")]
            impl TryFrom<EString> for $ty {
                type Error = <$ty as std::str::FromStr>::Err;

                #[inline]
                fn try_from(s: EString) -> Result<Self, Self::Error> {
                    s.0.parse::<Self>()
                }
            }
        )+
    };
}

#[rustfmt::skip]
from_env_string_numbers_impl![
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseError;

    #[test]
    fn should_parse_number() {
        let estr = EString::from("-10");
        match estr.parse::<i32>() {
            Ok(res) => assert_eq!(res, -10),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_parse_float_number() {
        let estr = EString::from("-0.15");
        match estr.parse::<f32>() {
            #[allow(clippy::float_cmp)]
            Ok(res) => assert_eq!(res, -0.15),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_parse_error() {
        let estr = EString::from("-10");
        match estr.parse::<u32>() {
            Err(ParseError(orig)) => {
                assert_eq!(orig, String::from("-10"));
            }
            _ => unreachable!(),
        };
    }
}
