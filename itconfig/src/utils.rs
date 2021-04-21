use crate::{EnvError, EnvString, FromEnvString};

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
