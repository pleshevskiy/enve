use crate::prelude::*;
use std::env;

/// This function is similar as `get_env`, but it unwraps result with panic on error.
///
/// Panics
/// ------
/// Application will panic if environment variable is missing or cannot parse variable to
/// expected type
///
pub fn get_env_or_panic<T>(env_name: &str) -> T
where
    T: FromEnvString,
{
    get_env(env_name).unwrap_or_else(make_panic)
}

/// Try to read environment variable and parse to expected type. You may to put to argument
/// any type with `FromEnvString` trait.
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
///     let result: bool = get_env("DEBUG").unwrap();
///
///     assert_eq!(result, true);
/// }
/// ```
///
pub fn get_env<T>(env_name: &str) -> Result<T, EnvError>
where
    T: FromEnvString,
{
    get_env_or(env_name, |_| {
        Err(EnvError::MissingVariable(env_name.to_string()))
    })
}

/// This function is similar as `get_env_or_panic`, but you can pass default value for
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
    get_env_or(env_name, |_| Ok(default.to_env_string())).unwrap_or_else(make_panic)
}

/// This function is similar as `get_env_or_default`, but the default value will be set to environment
/// variable, if env variable is missed.
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
    get_env_or(env_name, |_| {
        let val = default.to_env_string();
        env::set_var(env_name, val.as_str());
        Ok(val)
    })
    .unwrap_or_else(make_panic)
}

/// This function returns env variable as `EnvString` structure. You can pass callback for custom
/// default expression. Callback should return `EnvString` value or `EnvError`
pub fn get_env_or<T, F>(env_name: &str, cb: F) -> Result<T, EnvError>
where
    T: FromEnvString,
    F: FnOnce(env::VarError) -> Result<EnvString, EnvError>,
{
    env::var(env_name)
        .map(|s| s.to_env_string())
        .or_else(cb)
        .and_then(|env_str| parse_env_variable(env_name, env_str))
}

#[doc(hidden)]
fn parse_env_variable<T>(env_name: &str, env_str: EnvString) -> Result<T, EnvError>
where
    T: FromEnvString,
{
    env_str
        .parse::<T>()
        .map_err(|_| EnvError::FailedToParse(env_name.to_string()))
}

#[doc(hidden)]
fn make_panic<T>(e: EnvError) -> T {
    panic!("{}", e)
}
