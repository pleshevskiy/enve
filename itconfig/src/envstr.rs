use std::ops::Deref;

/// Wrapper under String type.
///
/// When we read the environment variable, we automatically convert the value
/// to EnvString and then convert it to your expected type.
///
#[derive(Debug, PartialEq, Clone)]
pub struct EnvString(String);

impl<T> From<T> for EnvString
where
    T: ToEnvString,
{
    fn from(val: T) -> Self {
        val.to_env_string()
    }
}

impl Deref for EnvString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A trait for converting value to EnvString.
///
/// This trait automatically implemented for any type which implements the
/// [`Display`] trait. As such, `ToEnvString` shouldn't be implemented directly:
/// [`Display`] should be implemented instead, and you get the `ToEnvString`
/// implementation for free.
///
/// [`Display`]: std::fmt::Display
pub trait ToEnvString {
    /// Converts the giving value to a `EnvString`.
    ///
    /// # Examples
    ///
    /// basic usage
    ///
    /// ```rust
    /// # use itconfig::{EnvString, ToEnvString};
    /// let i = 5;
    /// let five = EnvString::from("5");
    /// assert_eq!(five, i.to_env_string());
    /// ```
    fn to_env_string(&self) -> EnvString;
}

/// Simple and safe type conversions that may fail in a controlled way under
/// some circumstances.
///
/// This trait automatically implemented for all standard primitives. If you
/// want to use your custom type in the library you need to implement
/// `ToEnvString` and `FromEnvString` manually.
pub trait FromEnvString: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err>;
}

impl<T> ToEnvString for T
where
    T: std::fmt::Display,
{
    #[inline]
    fn to_env_string(&self) -> EnvString {
        EnvString(self.to_string())
    }
}

#[doc(hidden)]
macro_rules! from_env_string_numbers_impl {
    ($($ty:ty => $feature:expr),+) => {
        $(
            #[cfg(feature = $feature)]
            impl FromEnvString for $ty {
                type Err = <$ty as std::str::FromStr>::Err;

                #[inline]
                fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
                    s.0.parse::<Self>()
                }
            }
        )+
    };
}

from_env_string_numbers_impl![
    i8    => "i8",
    i16   => "i16",
    i32   => "i32",
    i64   => "i64",
    i128  => "i128",
    isize => "isize",
    u8    => "u8",
    u16   => "u16",
    u32   => "u32",
    u64   => "u64",
    u128  => "u128",
    usize => "usize",
    f32   => "f32",
    f64   => "f64"
];

#[cfg(feature = "bool")]
impl FromEnvString for bool {
    type Err = ();

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
            _ => Ok(false),
        }
    }
}

impl FromEnvString for String {
    type Err = ();

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        Ok(s.0.clone())
    }
}

impl FromEnvString for &'static str {
    type Err = ();

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        Ok(Box::leak(s.0.clone().into_boxed_str()))
    }
}

//===========================================================================//
// DEPRECATED                                                                //
//===========================================================================//

/// Error type for json array implementation
#[cfg(feature = "json_array")]
#[derive(Debug)]
#[deprecated(since = "1.1.0")]
pub enum ArrayEnvError {
    /// Invalid type.
    InvalidType,

    /// Failed to parse environment variable
    FailedToParse,
}

#[cfg(feature = "json_array")]
#[allow(deprecated)]
impl<T> FromEnvString for Vec<T>
where
    T: FromEnvString,
{
    type Err = ArrayEnvError;

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        serde_json::from_str::<Vec<isize>>(s.trim())
            .map(|vec| vec.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            .or_else(|_| serde_json::from_str::<Vec<String>>(s.trim()))
            .map_err(|_| ArrayEnvError::InvalidType)
            .and_then(|vec| {
                vec.iter()
                    .map(|v| {
                        FromEnvString::from_env_string(&v.to_env_string())
                            .map_err(|_| ArrayEnvError::FailedToParse)
                    })
                    .collect::<Result<Vec<T>, _>>()
            })
    }
}
