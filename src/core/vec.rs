use crate::core::EString;
use std::convert::TryFrom;
use std::fmt::Write;

pub const COMMA: char = ',';
pub const SEMI: char = ';';

pub type CommaVec<T> = SepVec<T, COMMA>;
pub type SemiVec<T> = SepVec<T, SEMI>;

#[derive(Debug, PartialEq, Clone)]
pub struct SepVec<T, const SEP: char>(pub Vec<T>);

impl<T, const SEP: char> std::ops::Deref for SepVec<T, SEP> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const SEP: char> From<Vec<T>> for SepVec<T, SEP> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T, const SEP: char> std::fmt::Display for SepVec<T, SEP>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().try_for_each(|(i, part)| {
            if i != 0 {
                f.write_char(SEP)?;
            }

            f.write_str(&part.to_string())
        })
    }
}

impl<T, const SEP: char> TryFrom<EString> for SepVec<T, SEP>
where
    T: TryFrom<EString> + std::fmt::Display,
{
    type Error = T::Error;

    fn try_from(value: EString) -> Result<Self, Self::Error> {
        let inner = value
            .split(SEP)
            .map(EString::from)
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(inner))
    }
}
