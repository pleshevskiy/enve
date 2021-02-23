use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EnvError {
    MissingVariable(String),
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
