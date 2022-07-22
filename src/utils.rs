use crate::core::EString;
use crate::error::Error;
use std::convert::{TryFrom, TryInto};
use std::env;

pub fn get_env_or_set_default<R>(env_name: &str, default: R) -> Result<R, Error>
where
    R: TryFrom<EString> + std::fmt::Display,
{
    get_env::<R>(env_name).or_else(|err| match err {
        Error::NotPresent => {
            let val = default.to_string();
            env::set_var(env_name, &val);
            EString::from(val)
                .try_into()
                .map_err(|_| Error::Parse(default.to_string()))
        }
        _ => Err(err),
    })
}

pub fn get_env<R>(env_name: &str) -> Result<R, Error>
where
    R: TryFrom<EString>,
{
    env::var(env_name)
        .map_err(From::from)
        .map(EString::from)
        .and_then(|val| {
            val.clone()
                .try_into()
                .map_err(|_| Error::Parse(val.to_string()))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<const N: u8>;

    impl<const N: u8> std::fmt::Display for TestCase<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "get_env_{}", N)
        }
    }

    #[test]
    fn should_return_variable() {
        let en = TestCase::<1>.to_string();
        env::set_var(&en, "hello");
        match get_env::<&str>(&en) {
            Ok(res) => assert_eq!(res, "hello"),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_no_present_error() {
        let en = TestCase::<2>.to_string();
        match get_env::<&str>(&en) {
            Err(Error::NotPresent) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_set_default_if_var_is_no_present() {
        let en = TestCase::<3>.to_string();
        let orig = "hello";
        match get_env_or_set_default(&en, orig) {
            Ok(res) => {
                assert_eq!(res, orig);
                assert_eq!(env::var(&en).unwrap(), orig);
            }
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "number")]
    mod numbers {
        use super::*;

        #[test]
        fn should_throw_parse_error() {
            let en = TestCase::<4>.to_string();
            env::set_var(&en, "-10");
            match get_env::<u32>(&en) {
                Err(Error::Parse(orig)) => {
                    assert_eq!(orig, String::from("-10"))
                }
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_set_default_num_if_var_is_no_present() {
            let en = TestCase::<5>.to_string();
            let orig = 10;
            match get_env_or_set_default(&en, orig) {
                Ok(res) => {
                    assert_eq!(res, orig);
                    assert_eq!(env::var(&en).unwrap(), "10");
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
            let en = TestCase::<5>.to_string();

            [
                ("y", true),
                ("yes", true),
                ("true", true),
                ("t", true),
                ("on", true),
                ("false", false),
                ("f", false),
            ]
            .iter()
            .for_each(|(val, expected)| {
                let mut en = en.clone();
                en.push_str(val.as_ref());

                env::set_var(&en, val);
                match get_env::<bool>(&en) {
                    Ok(res) => assert_eq!(res, *expected),
                    _ => unreachable!(),
                };
            })
        }
    }

    #[cfg(feature = "vec")]
    mod vector {
        use super::*;
        use crate::core::vec::{CommaVec, SepVec};

        #[test]
        fn should_return_var_as_vector() {
            let en = TestCase::<6>.to_string();

            env::set_var(&en, "1,2,3,4,5");
            match get_env::<CommaVec<i32>>(&en) {
                Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_throw_parse_vec_error() {
            let en = TestCase::<7>.to_string();
            env::set_var(&en, "1,2,3,4,5");
            match get_env::<SepVec<i32, '+'>>(&en) {
                Err(Error::Parse(orig)) => {
                    assert_eq!(orig, String::from("1,2,3,4,5"))
                }
                _ => unreachable!(),
            };
        }

        #[test]
        fn should_set_default_vector_if_var_is_no_present() {
            let en = TestCase::<8>.to_string();
            let orig = CommaVec::from(vec![1, 2, 3, 4]);
            match get_env_or_set_default(&en, orig.clone()) {
                Ok(res) => {
                    assert_eq!(res, orig);
                    assert_eq!(env::var(&en).unwrap(), "1,2,3,4");
                }
                _ => unreachable!(),
            };
        }
    }
}
