use failure::Fail;


#[derive(Debug, Fail, PartialEq)]
pub enum EnvError {
    #[fail(display = r#"Environment variable "{}" is missing"#, env_name)]
    MissingVariable { env_name: String },
    #[fail(display = r#"Failed to parse environment variable "{}""#, env_name)]
    FailedToParse { env_name: String },
}
