use crate::core::EString;
use std::convert::TryFrom;

impl TryFrom<EString> for bool {
    type Error = ();

    #[inline]
    fn try_from(s: EString) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
            "false" | "f" | "no" | "n" | "off" | "0" | "" => Ok(false),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseError;

    #[test]
    fn should_parse_bool_variable() {
        let test_cases = [
            ("1", true),
            ("0", false),
            ("y", true),
            ("f", false),
            ("yes", true),
            ("true", true),
            ("false", false),
            ("t", true),
            ("f", false),
            ("on", true),
            ("off", false),
        ];

        for (val, expected) in test_cases {
            let estr = EString::from(val);
            match estr.parse::<bool>() {
                Ok(res) => assert_eq!(res, expected),
                _ => unreachable!(),
            };
        }
    }

    #[test]
    fn should_throw_parse_error() {
        let estr = EString::from("something");
        match estr.parse::<bool>() {
            Err(ParseError(orig)) => {
                assert_eq!(orig, String::from("something"));
            }
            _ => unreachable!(),
        };
    }
}
