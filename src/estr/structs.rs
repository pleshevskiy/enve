use estring::SepVec;

const COMMA: char = ',';
const SEMI: char = ';';

/// Splits substring by comma character and returns ``SepVec``
pub type CommaVec<T> = SepVec<T, COMMA>;

/// Splits substring by semicolon character and returns ``SepVec``
pub type SemiVec<T> = SepVec<T, SEMI>;
