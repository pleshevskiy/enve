use crate::error::Error;
use estring::EString;
use std::convert::TryFrom;

/// Fetches the environment variable `key` from the current process. It set value `default`
/// if environment variable `key` ins'n set. Then this function tries to parse ``EString`` to
/// expected type by annotations.
///
/// # Errors
///
/// This function will return an error if ``EString`` cannot parse substring.
///
/// This function may return an error if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// This function will return an error if the environment variable's value is
/// not valid Unicode. If this is not desired, consider using [`var_os`].
///
/// # Examples
///
/// ```rust
/// let key = "doc_get_or_set";
/// match enve::get_or_set_default::<i32>(key, 10) {
///     Ok(res) => assert_eq!(res, 10),
///     Err(e) => println!("couldn't interpret {}: {}", key, e),
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn get_or_set_default<R>(env_name: &str, default: R) -> Result<R, Error>
where
    R: TryFrom<EString> + std::fmt::Display,
{
    get::<R>(env_name).or_else(|err| match err {
        Error::NotPresent => sset(env_name, &default).parse().map_err(Error::from),
        _ => Err(err),
    })
}

/// Fetches the environment variable `key` from the current process and then tries to parse
/// ``EString`` to expected type by annotations.
///
/// # Errors
///
/// This function will return an error if ``EString`` cannot parse substring.
///
/// This function will return an error if the environment variable isn't set.
///
/// This function may return an error if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// This function will return an error if the environment variable's value is
/// not valid Unicode. If this is not desired, consider using [`var_os`].
///
/// # Examples
///
/// ```rust
/// let key = "doc_get";
/// enve::sset(key, "10");
/// match enve::get::<i32>(key) {
///     Ok(res) => assert_eq!(res, 10),
///     Err(e) => println!("couldn't interpret {}: {}", key, e),
/// }
/// ```
pub fn get<R>(key: &str) -> Result<R, Error>
where
    R: TryFrom<EString>,
{
    sget(key).and_then(|v| v.parse().map_err(Error::from))
}

/// Fetches the environment variable `key` from the current process and returns value as
/// ``EString``.
///
/// # Errors
///
/// This function will return an error if the environment variable isn't set.
///
/// This function may return an error if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// This function will return an error if the environment variable's value is
/// not valid Unicode. If this is not desired, consider using [`var_os`].
///
/// # Examples
///
/// ```rust
/// let key = "HOME";
/// match enve::sget(key) {
///     Ok(val) => println!("{}: {:?}", key, val),
///     Err(e) => println!("couldn't interpret {}: {}", key, e),
/// }
/// ```
pub fn sget(key: &str) -> Result<EString, Error> {
    std::env::var(key).map_err(Error::from).map(EString::from)
}

/// Sets the environment variable `key` to the value `value` for the currently running
/// process and then returns `value` as a ``EString``.
///
/// # Panics
///
/// This function may panic if `key` is empty, contains an ASCII equals sign `'='`
/// or the NUL character `'\0'`, or when `value` contains the NUL character.
///
/// # Examples
///
/// ```
/// let estr = enve::sset("KEY", "10");
/// assert_eq!(estr.to_string(), String::from("10"));
/// ```
pub fn sset<V>(key: &str, value: V) -> EString
where
    V: std::fmt::Display,
{
    let val = value.to_string();
    std::env::set_var(key, &val);
    val.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<const N: u8>;

    impl<const N: u8> std::fmt::Display for TestCase<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "test_case_{}", N)
        }
    }

    #[test]
    fn should_add_env_variable_to_process() {
        let en = TestCase::<0>.to_string();
        sset(&en, "hello");
        match std::env::var(&en) {
            Ok(var) => assert_eq!(&var, "hello"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_return_variable() {
        let en = TestCase::<1>.to_string();
        std::env::set_var(&en, "hello");
        match get::<&str>(&en) {
            Ok(res) => assert_eq!(res, "hello"),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_no_present_error() {
        let en = TestCase::<2>.to_string();
        match get::<&str>(&en) {
            Err(Error::NotPresent) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_set_default_if_var_is_no_present() {
        let en = TestCase::<3>.to_string();
        let orig = "hello";
        match get_or_set_default(&en, orig) {
            Ok(res) => {
                assert_eq!(res, orig);
                assert_eq!(std::env::var(&en).unwrap(), orig);
            }
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "number")]
    mod numbers {
        use super::*;

        #[test]
        fn should_return_parsed_num() {
            let en = TestCase::<4>.to_string();
            std::env::set_var(&en, "-10");
            match get::<i32>(&en) {
                Ok(res) => assert_eq!(res, -10),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_throw_parse_error() {
            let en = TestCase::<5>.to_string();
            std::env::set_var(&en, "-10");
            match get::<u32>(&en) {
                Err(Error::Parse(orig)) => {
                    assert_eq!(orig, String::from("-10"));
                }
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_set_default_num_if_var_is_no_present() {
            let en = TestCase::<6>.to_string();
            let orig = 10;
            match get_or_set_default(&en, orig) {
                Ok(res) => {
                    assert_eq!(res, orig);
                    assert_eq!(std::env::var(&en).unwrap(), "10");
                }
                _ => unreachable!(),
            };
        }
    }

    #[cfg(feature = "bool")]
    mod boolean {
        use super::*;

        #[test]
        fn should_parse_bool_variable() {
            let en = TestCase::<7>.to_string();

            let test_cases = [
                ("1", true),
                ("y", true),
                ("yes", true),
                ("true", true),
                ("t", true),
                ("on", true),
                ("false", false),
                ("f", false),
                ("0", false),
            ];
            for (val, expected) in &test_cases {
                let mut en = en.clone();
                en.push_str(val.as_ref());

                std::env::set_var(&en, val);
                match get::<bool>(&en) {
                    Ok(res) => assert_eq!(res, *expected),
                    _ => unreachable!(),
                };
            }
        }
    }

    #[cfg(feature = "vec")]
    mod vector {
        use super::*;
        use crate::estr::{CommaVec, SemiVec, SepVec};

        #[test]
        fn should_return_var_as_vector() {
            let en = TestCase::<8>.to_string();

            std::env::set_var(&en, "1,2,3,4,5");
            match get::<CommaVec<i32>>(&en) {
                Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_trim_identations_before_parsing() {
            let en = TestCase::<9>.to_string();

            let input = "
1 , 2, 3,
4,5";

            std::env::set_var(&en, input);
            match get::<CommaVec<i32>>(&en) {
                Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_return_vector_of_vectors() {
            let en = TestCase::<10>.to_string();

            std::env::set_var(&en, "1,2; 3,4,5; 6,7");
            match get::<SemiVec<CommaVec<i32>>>(&en) {
                Ok(res) => assert_eq!(
                    res,
                    SemiVec::from(vec![
                        CommaVec::from(vec![1, 2]),
                        CommaVec::from(vec![3, 4, 5]),
                        CommaVec::from(vec![6, 7])
                    ])
                ),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_throw_parse_vec_error() {
            let en = TestCase::<11>.to_string();
            std::env::set_var(&en, "1,2,3,4,5");
            match get::<SepVec<i32, '+'>>(&en) {
                Err(Error::Parse(orig)) => {
                    assert_eq!(orig, String::from("1,2,3,4,5"));
                }
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_set_default_vector_if_var_is_no_present() {
            let en = TestCase::<12>.to_string();
            let orig = CommaVec::from(vec![1, 2, 3, 4]);
            match get_or_set_default(&en, orig.clone()) {
                Ok(res) => {
                    assert_eq!(res, orig);
                    assert_eq!(std::env::var(&en).unwrap(), "1,2,3,4");
                }
                _ => unreachable!(),
            };
        }
    }
}
