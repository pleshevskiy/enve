use std::ops::Deref;


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
            _ => Ok(false)
        }
    }
}



#[cfg(feature = "array")]
pub enum ArrayEnvError {
    InvalidType,
    FailedToParse,
}


#[cfg(feature = "array")]
impl<T> FromEnvString for Vec<T>
    where T: FromEnvString
{
    type Err = ArrayEnvError;

    fn from_env_string(s: &EnvString) -> Result<Self, Self::Err> {
        serde_json::from_str::<Vec<isize>>(s.trim())
            .map(|vec| {
                vec.iter().map(|v| v.to_string()).collect::<Vec<String>>()
            })
            .or_else(|_| {
                serde_json::from_str::<Vec<String>>(s.trim())
            })
            .map_err(|_| ArrayEnvError::InvalidType)
            .and_then(|vec| {
                vec.iter()
                    .map(|v| {
                        v.to_env_string()
                            .parse::<T>()
                            .map_err(|_| ArrayEnvError::FailedToParse)
                    })
                    .collect::<Result<Vec<T>, _>>()
            })
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


