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
    #[cfg(feature = "vec")]
    use crate::core::vec::{CommaVec, SepVec};

    #[test]
    fn should_return_variable() {
        env::set_var("get_env_1", "hello");
        match get_env::<&str>("get_env_1") {
            Ok(res) => assert_eq!(res, "hello"),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_no_present_error() {
        match get_env::<&str>("get_env_2") {
            Err(Error::NotPresent) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_throw_parse_error() {
        env::set_var("get_env_3", "-10");
        match get_env::<u32>("get_env_3") {
            Err(Error::Parse(orig)) => {
                assert_eq!(orig, String::from("-10"))
            }
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_set_default_if_var_is_no_present() {
        let orig = 10;
        match get_env_or_set_default("get_env_4", orig) {
            Ok(res) => {
                assert_eq!(res, orig);
                assert_eq!(env::var("get_env_4").unwrap(), "10");
            }
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "vec")]
    #[test]
    fn should_return_var_as_vector() {
        env::set_var("get_env_5", "1,2,3,4,5");
        match get_env::<CommaVec<i32>>("get_env_5") {
            Ok(res) => assert_eq!(*res, vec![1, 2, 3, 4, 5]),
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "vec")]
    #[test]
    fn should_throw_parse_vec_error() {
        env::set_var("get_env_6", "1,2,3,4,5");
        match get_env::<SepVec<i32, '+'>>("get_env_6") {
            Err(Error::Parse(orig)) => {
                assert_eq!(orig, String::from("1,2,3,4,5"))
            }
            _ => unreachable!(),
        };
    }

    #[cfg(feature = "vec")]
    #[test]
    fn should_set_default_vector_if_var_is_no_present() {
        let orig = CommaVec::from(vec![1, 2, 3, 4]);
        match get_env_or_set_default("get_env_7", orig.clone()) {
            Ok(res) => {
                assert_eq!(res, orig);
                assert_eq!(env::var("get_env_7").unwrap(), "1,2,3,4");
            }
            _ => unreachable!(),
        };
    }
}
