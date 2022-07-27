use std::env::VarError;
use std::error;
use std::ffi::OsString;
use std::fmt;

use estring::EString;

/// The error type for operations interacting with environment variables
#[derive(Debug)]
pub enum Error {
    /// The specified environment variable was not present in the current process's environment.
    NotPresent,

    /// Failed to parse the specified environment variable.
    Parse(EString),

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    Invalid(OsString),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::NotPresent => f.write_str("The specified env variable was not present"),
            Error::Invalid(inner) => write!(
                f,
                "The specified env variable was found, but it did not valid: '{:?}'",
                inner,
            ),
            Error::Parse(env_name) => {
                write!(f, r#"Failed to parse environment variable "{}""#, env_name)
            }
        }
    }
}

impl error::Error for Error {}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        match err {
            VarError::NotPresent => Error::NotPresent,
            VarError::NotUnicode(inner) => Error::Invalid(inner),
        }
    }
}

impl From<estring::Error> for Error {
    fn from(err: estring::Error) -> Self {
        Error::Parse(err.0)
    }
}
