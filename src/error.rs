use std::env::VarError;
use std::error;
use std::ffi::OsString;
use std::fmt;

use estring::EString;

/// The error type for operations interacting with environment variables
#[derive(Debug)]
pub struct Error(pub(crate) String, pub(crate) Reason);

impl Error {
    /// Returns the environment variable name for the failure
    pub fn var_name(&self) -> &str {
        &self.0
    }

    /// Returns the reason for the failure
    pub fn reason(&self) -> &Reason {
        &self.1
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The environment variable '{}' failed: {}",
            self.0, self.1
        )
    }
}

/// The reason for the failure to get environment variable
#[derive(Debug, Clone)]
pub enum Reason {
    /// The specified environment variable was not present in the current process's environment.
    NotPresent,

    /// Failed to parse the specified environment variable.
    Parse(EString),

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    Invalid(OsString),
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Reason::NotPresent => f.write_str("The specified env variable was not present"),
            Reason::Invalid(inner) => write!(
                f,
                "The specified env variable was found, but it did not valid: '{:?}'",
                inner,
            ),
            Reason::Parse(env_name) => {
                write!(f, r#"Failed to parse environment variable "{}""#, env_name)
            }
        }
    }
}

impl error::Error for Error {}

impl From<VarError> for Reason {
    fn from(err: VarError) -> Self {
        match err {
            VarError::NotPresent => Reason::NotPresent,
            VarError::NotUnicode(inner) => Reason::Invalid(inner),
        }
    }
}

impl From<estring::Error> for Reason {
    fn from(err: estring::Error) -> Self {
        Reason::Parse(err.0)
    }
}
