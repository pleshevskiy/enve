use std::ops::Deref;
use std::str::FromStr;


#[doc(hidden)]
pub trait ToEnvString {
    fn to_env_string(&self) -> EnvString;
}


#[doc(hidden)]
pub trait FromEnvString: Sized {
    type Err;

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err>;
}


impl<T> ToEnvString for T
    where
        T: ToString
{
    #[inline]
    fn to_env_string(&self) -> EnvString {
        EnvString(self.to_string())
    }
}


#[doc(hidden)]
macro_rules! from_env_string_numbers_impl {
    ($($ty:ty),+) => {
        $(
            impl FromEnvString for $ty {
                type Err = <$ty as FromStr>::Err;

                #[inline]
                fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
                    s.0.parse::<Self>()
                }
            }
        )+
    };
}

from_env_string_numbers_impl![
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
];


impl FromEnvString for bool {
    type Err = ();

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
            _ => Ok(false)
        }
    }
}


impl FromEnvString for String {
    type Err = ();

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        Ok(s.0.clone())
    }
}


#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub struct EnvString(String);

impl EnvString {
    pub fn parse<T: FromEnvString>(&self) -> Result<T, T::Err> {
        FromEnvString::from_env_string(self)
    }
}

impl Deref for EnvString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


