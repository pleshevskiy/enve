use crate::{EnvError, EnvString, FromEnvString, ToEnvString};

pub(crate) fn parse_env_variable<T>(env_name: &str, env_str: EnvString) -> Result<T, EnvError>
where
    T: FromEnvString,
{
    FromEnvString::from_env_string(&env_str)
        .map_err(|_| EnvError::FailedToParse(env_name.to_string()))
}

pub(crate) fn make_panic<T>(e: EnvError) -> T {
    panic!("{}", e)
}

pub(crate) fn join(env_strings: &[EnvString], sep: &str) -> String {
    env_strings
        .iter()
        .enumerate()
        .fold(String::new(), |mut res, (i, item)| {
            if i > 0 {
                res.push_str(sep);
            }
            res.push_str(item);
            res
        })
}

pub(crate) fn vec_to_env_strings<T>(values: Vec<T>) -> Vec<EnvString>
where
    T: ToEnvString,
{
    values.into_iter().map(EnvString::from).collect()
}
