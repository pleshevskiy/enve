use crate::core::EString;
use std::convert::{Infallible, TryFrom};

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

#[cfg(feature = "bool")]
impl TryFrom<EString> for bool {
    type Error = Infallible;

    #[inline]
    fn try_from(s: EString) -> Result<Self, Self::Error> {
        Ok(matches!(
            s.to_lowercase().as_str(),
            "true" | "t" | "yes" | "y" | "on" | "1"
        ))
    }
}
