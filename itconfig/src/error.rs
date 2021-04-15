use std::error;
use std::fmt;

/// The error type for operations interacting with environment variables
#[derive(Debug, PartialEq)]
pub enum EnvError {
    /// The specified environment variable was not present in the current process's environment.
    MissingVariable(String),

    /// Failed to parse the specified environment variable.
    FailedToParse(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EnvError::MissingVariable(env_name) => {
                write!(f, r#"Environment variable "{}" is missing"#, env_name)
            }
            EnvError::FailedToParse(env_name) => {
                write!(f, r#"Failed to parse environment variable "{}""#, env_name)
            }
        }
    }
}

impl error::Error for EnvError {}
