use crate::envstr::*;
use crate::error::*;
use crate::utils::*;
use std::env;

/// Same as get_env but returns Option enum instead Result
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::maybe_get_env;
/// use std::env;
///
/// fn main () {
///     env::set_var("HOST", "https://example.com");
///
///     let host: Option<&'static str> = maybe_get_env("HOST");
///     let not_existence_host: Option<&'static str> = maybe_get_env("NOT_EXISTENCE_HOST");
///
///     assert_eq!(host, Some("https://example.com"));
///     assert_eq!(not_existence_host, None);
/// }
/// ```
///
pub fn maybe_get_env<T>(env_name: &str) -> Option<T>
where
    T: FromEnvString,
{
    get_env(env_name).ok()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Environment variable \"TEST_CASE_1\" is missing")]
    fn get_missing_env() {
        get_env_or_panic::<String>("TEST_CASE_1");
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_2\"")]
    fn get_env_with_invalid_value() {
        let env_name = "TEST_CASE_2";
        env::set_var(&env_name, "30r");
        get_env_or_panic::<u32>(env_name);
    }

    #[test]
    fn get_result_of_missing_env() {
        let env_name = String::from("TEST_CASE_3");
        let env_val = get_env::<String>(&env_name);
        assert_eq!(env_val, Err(EnvError::MissingVariable(env_name)))
    }

    #[test]
    fn get_result_of_env_with_invalid_value() {
        let env_name = String::from("TEST_CASE_4");
        env::set_var(&env_name, "30r");
        let env_val = get_env::<u32>(&env_name);
        assert_eq!(env_val, Err(EnvError::FailedToParse(env_name)))
    }

    #[test]
    fn get_result_of_env_successfully() {
        env::set_var("TEST_CASE_5", "30");
        let env_var = get_env("TEST_CASE_5");
        assert_eq!(env_var, Ok(30));
    }

    #[test]
    fn get_missing_env_with_default_value() {
        let flag: bool = get_env_or_default("TEST_CASE_6", "true");
        assert!(flag);
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_7\"")]
    fn get_invalid_env_with_default_value() {
        env::set_var("TEST_CASE_7", "30r");
        get_env_or_default::<u32, _>("TEST_CASE_7", 30);
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_8\"")]
    fn get_env_with_invalid_default_value() {
        get_env_or_default::<u32, _>("TEST_CASE_8", "30r");
    }

    #[test]
    fn get_env_with_default_successfully() {
        env::set_var("TEST_CASE_9", "10");
        let env_val: u32 = get_env_or_default("TEST_CASE_9", 30);
        assert_eq!(env_val, 10)
    }

    #[test]
    fn get_missing_env_with_set_default_value() {
        let flag: bool = get_env_or_set_default("TEST_CASE_10", "true");
        assert!(flag);

        let env_var = env::var("TEST_CASE_10");
        assert_eq!(env_var, Ok(String::from("true")))
    }

    #[test]
    fn get_optional_env() {
        env::set_var("TEST_CASE_11", "something");
        let something: Option<&'static str> = maybe_get_env("TEST_CASE_11");
        assert_eq!(something, Some("something"));

        let nothing: Option<&'static str> = maybe_get_env("TEST_CASE_11_NONE");
        assert_eq!(nothing, None);
    }
}
