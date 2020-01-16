use std::env;
use crate::envstr::{EnvString, FromEnvString, ToEnvString};


#[doc(hidden)]
macro_rules! env_panic {
    (MissingVariable, $env_name:expr) => {
        panic!(r#"Environment variable "{}" is missing"#, $env_name)
    };
    (FailedToParse, $env_name:expr) => {
        panic!(r#"Failed to parse environment variable "{}""#, $env_name)
    };
}


/// Try to read environment variable and parse to expected type. You may to put to argument
/// any type with `FromEnvString` trait.
///
/// Panics
/// ------
/// Application will panic if environment variable is missing or cannot parse variable to
/// expected type
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::get_env;
/// use std::env;
///
/// fn main () {
///     env::set_var("DEBUG", "true");
///
///     let result: bool = get_env("DEBUG");
///
///     assert_eq!(result, true);
/// }
/// ```
///
pub fn get_env<T>(env_name: &str) -> T
    where
        T: FromEnvString
{
    get_env_or(env_name, || env_panic!(MissingVariable, env_name))
}


/// This function is similar as `get_env` but more safely. You can pass default value for
/// environment variable with `ToEnvString` trait.
///
/// Panics
/// ------
/// Application will panic if cannot parse variable to expected type
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::get_env_or_default;
/// use std::env;
///
/// fn main () {
///     let result: bool = get_env_or_default("TESTING", "true");
///     assert_eq!(result, true);
/// }
/// ```
///
pub fn get_env_or_default<T, D>(env_name: &str, default: D) -> T
    where
        T: FromEnvString,
        D: ToEnvString,
{
    get_env_or(env_name, || default.to_env_string())
}


/// This function is similar as `get_env_or_default` but if env variable is missed, will set
/// default value to environment variable.
///
/// Panics
/// ------
/// Application will panic if cannot parse variable to expected type
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::get_env_or_set_default;
/// use std::env;
///
/// fn main () {
///     let result: bool = get_env_or_set_default("TESTING", "true");
///     assert_eq!(result, true);
///
///     let var = env::var("TESTING").unwrap();
///     assert_eq!(var, "true");
/// }
/// ```
///
pub fn get_env_or_set_default<T, D>(env_name: &str, default: D) -> T
    where
        T: FromEnvString,
        D: ToEnvString,
{
    get_env_or(env_name, || {
        let val = default.to_env_string();
        env::set_var(env_name, val.as_str());
        val
    })
}


/// This function returns env variable as `EnvString` structure. You can pass callback for custom
/// default expression. Callback should return `EnvString` value or `panic!`
pub fn get_env_or<T, F>(env_name: &str, cb: F) -> T
    where
        T: FromEnvString,
        F: FnOnce() -> EnvString
{
    let env_str = env::var(env_name)
        .map(|s| s.to_env_string())
        .unwrap_or_else(|_| cb());

    parse_env_variable(env_name, env_str)
}



#[doc(hidden)]
fn parse_env_variable<T>(env_name: &str, env_str: EnvString) -> T
    where
        T: FromEnvString
{
    env_str
        .parse::<T>()
        .unwrap_or_else(|_| env_panic!(FailedToParse, env_name))
}

