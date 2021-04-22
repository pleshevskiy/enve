use crate::envstr::*;
use crate::error::*;
use crate::utils::*;
use std::env;

/// Same as get_vec_env but returns Option enum instead Result
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::*;
/// use std::env;
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum PaymentPlatform {
///     PayPal,
///     Stripe,
///     SomethingElse,
/// }
///
/// impl FromEnvString for PaymentPlatform {
///     type Err = &'static str;
///
///     fn from_env_string(envstr: &EnvString) -> Result<Self, Self::Err> {
///         match envstr.to_lowercase().as_str() {
///             "paypal" => Ok(Self::PayPal),
///             "stripe" => Ok(Self::Stripe),
///             "smth" => Ok(Self::SomethingElse),
///             _ => Err("Unsupported payment platform"),
///         }
///     }
/// }
///
///
/// fn main () {
///     env::set_var("PAYMENT_PLATFORMS", "paypal,stripe");
///
///     let payment_platforms: Option<Vec<PaymentPlatform>> = maybe_get_vec_env("PAYMENT_PLATFORMS", ",");
///     assert_eq!(
///         payment_platforms,
///         Some(vec![PaymentPlatform::PayPal, PaymentPlatform::Stripe])
///     );
/// }
/// ```
///
pub fn maybe_get_vec_env<T>(env_name: &str, sep: &'static str) -> Option<Vec<T>>
where
    T: FromEnvString,
{
    get_vec_env(env_name, sep).ok()
}

/// This function is similar as `get_vec_env`, but it unwraps result with panic on error.
///
/// Panics
/// ------
/// Application will panic if environment variable is missing or cannot parse variable to
/// expected type
///
pub fn get_vec_env_or_panic<T>(env_name: &str, sep: &'static str) -> Vec<T>
where
    T: FromEnvString,
{
    get_vec_env(env_name, sep).unwrap_or_else(make_panic)
}

/// Try to read environment variable, split by separator and parse each item to expected
/// type.
///
/// Example
/// -------
///
/// ```rust
/// # extern crate itconfig;
/// # use itconfig::get_vec_env;
/// use std::env;
///
/// fn main () {
///     env::set_var("DEBUG", "true");
///
///     let result: Vec<bool> = get_vec_env("DEBUG", ",").unwrap();
///
///     assert_eq!(result, vec![true]);
/// }
/// ```
///
pub fn get_vec_env<T>(env_name: &str, sep: &str) -> Result<Vec<T>, EnvError>
where
    T: FromEnvString,
{
    get_vec_env_or(env_name, sep, |_| {
        Err(EnvError::MissingVariable(env_name.to_string()))
    })
}

/// This function is similar as `get_vec_env_or_panic`, but you can pass default value for
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
/// # use itconfig::get_vec_env_or_default;
/// use std::env;
///
/// fn main () {
///     let result: Vec<bool> = get_vec_env_or_default("TESTING", ",", vec!["true"]);
///     assert_eq!(result, vec![true]);
/// }
/// ```
///
pub fn get_vec_env_or_default<T, D>(env_name: &str, sep: &str, default: Vec<D>) -> Vec<T>
where
    T: FromEnvString,
    D: ToEnvString,
{
    get_vec_env_or(env_name, sep, |_| Ok(vec_to_env_strings(default))).unwrap_or_else(make_panic)
}

/// This function is similar as `get_vec_env_or_default`, but the default value will be set to environment
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
/// # use itconfig::get_vec_env_or_set_default;
/// use std::env;
///
/// fn main () {
///     let result: Vec<bool> = get_vec_env_or_set_default("TESTING", ",", vec!["true"]);
///     assert_eq!(result, vec![true]);
///
///     let var = env::var("TESTING").unwrap();
///     assert_eq!(var, "true");
/// }
/// ```
///
pub fn get_vec_env_or_set_default<T, D>(env_name: &str, sep: &str, default: Vec<D>) -> Vec<T>
where
    T: FromEnvString,
    D: ToEnvString,
{
    get_vec_env_or(env_name, sep, |_| {
        let default_env_strings = vec_to_env_strings(default);
        let env_val = join(&default_env_strings, sep);
        env::set_var(env_name, env_val.as_str());
        Ok(default_env_strings)
    })
    .unwrap_or_else(make_panic)
}

/// This function returns env variable as `EnvString` structure. You can pass callback for custom
/// default expression. Callback should return `EnvString` value or `EnvError`
pub fn get_vec_env_or<T, F>(env_name: &str, sep: &str, cb: F) -> Result<Vec<T>, EnvError>
where
    T: FromEnvString,
    F: FnOnce(env::VarError) -> Result<Vec<EnvString>, EnvError>,
{
    env::var(env_name)
        .map(|s| {
            s.split(sep)
                .into_iter()
                .map(|item| item.to_env_string())
                .collect()
        })
        .or_else(cb)
        .and_then(|items| {
            items
                .into_iter()
                .map(|env_str| parse_env_variable(env_name, env_str))
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEP: &str = ",";

    #[test]
    #[should_panic(expected = "Environment variable \"TEST_CASE_VEC_1\" is missing")]
    fn get_missing_vec_env() {
        let _: Vec<&'static str> = get_vec_env_or_panic("TEST_CASE_VEC_1", SEP);
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_VEC_2\"")]
    fn get_vec_env_with_invalid_value() {
        let env_name = "TEST_CASE_VEC_2";
        env::set_var(&env_name, "30r");
        let _: Vec<u32> = get_vec_env_or_panic(env_name, SEP);
    }

    #[test]
    fn get_result_of_missing_vec_env() {
        let env_name = String::from("TEST_CASE_VEC_3");
        let env_val = get_vec_env::<String>(&env_name, SEP);
        assert_eq!(env_val, Err(EnvError::MissingVariable(env_name)))
    }

    #[test]
    fn get_result_of_vec_env_with_invalid_value() {
        let env_name = String::from("TEST_CASE_VEC_4");
        env::set_var(&env_name, "30r");
        let env_val = get_vec_env::<u32>(&env_name, SEP);
        assert_eq!(env_val, Err(EnvError::FailedToParse(env_name)))
    }

    #[test]
    fn get_result_of_vec_env_successfully() {
        env::set_var("TEST_CASE_VEC_5", "30");
        let env_var = get_vec_env("TEST_CASE_VEC_5", SEP);
        assert_eq!(env_var, Ok(vec![30]));
    }

    #[test]
    fn get_missing_vec_env_with_default_value() {
        let flag: Vec<bool> = get_vec_env_or_default("TEST_CASE_VEC_6", SEP, vec!["true"]);
        assert_eq!(flag, vec![true]);
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_VEC_7\"")]
    fn get_invalid_vec_env_with_default_value() {
        env::set_var("TEST_CASE_VEC_7", "30r");
        get_vec_env_or_default::<u32, _>("TEST_CASE_VEC_7", SEP, vec![30]);
    }

    #[test]
    #[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_VEC_8\"")]
    fn get_vec_env_with_invalid_default_value() {
        get_vec_env_or_default::<u32, _>("TEST_CASE_VEC_8", SEP, vec!["30r"]);
    }

    #[test]
    fn get_vec_env_with_default_successfully() {
        env::set_var("TEST_CASE_VEC_9", "10");
        let env_val: Vec<u32> = get_vec_env_or_default("TEST_CASE_VEC_9", SEP, vec![30]);
        assert_eq!(env_val, vec![10])
    }

    #[test]
    fn get_missing_vec_env_with_set_default_value() {
        let flag: Vec<bool> = get_vec_env_or_set_default("TEST_CASE_VEC_10", SEP, vec!["true"]);
        assert_eq!(flag, vec![true]);

        let env_var = env::var("TEST_CASE_VEC_10");
        assert_eq!(env_var, Ok(String::from("true")))
    }

    #[test]
    fn get_optional_vec_env() {
        env::set_var("TEST_CASE_VEC_11", "something");
        let something: Option<Vec<&'static str>> = maybe_get_vec_env("TEST_CASE_VEC_11", SEP);
        assert_eq!(something, Some(vec!["something"]));

        let nothing: Option<Vec<&'static str>> = maybe_get_vec_env("TEST_CASE_VEC_11_NONE", SEP);
        assert_eq!(nothing, None);
    }

    #[test]
    fn get_custom_type_from_vec_env() {
        #[derive(Debug, PartialEq, Eq)]
        enum PaymentPlatform {
            PayPal,
            Stripe,
            SomethingElse,
        }

        impl FromEnvString for PaymentPlatform {
            type Err = &'static str;

            fn from_env_string(envstr: &EnvString) -> Result<Self, Self::Err> {
                match envstr.to_lowercase().as_str() {
                    "paypal" => Ok(Self::PayPal),
                    "stripe" => Ok(Self::Stripe),
                    "smth" => Ok(Self::SomethingElse),
                    _ => Err("Unsupported payment platform"),
                }
            }
        }

        env::set_var("TEST_CASE_VEC_12", "paypal,stripe");
        let something: Option<Vec<PaymentPlatform>> = maybe_get_vec_env("TEST_CASE_VEC_12", SEP);
        assert_eq!(
            something,
            Some(vec![PaymentPlatform::PayPal, PaymentPlatform::Stripe])
        );
    }
}
