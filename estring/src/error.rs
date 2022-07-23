/// Failed to parse the specified string.
#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Failed to parse: "{}""#, self.0)
    }
}

impl std::error::Error for ParseError {}

impl std::ops::Deref for ParseError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
