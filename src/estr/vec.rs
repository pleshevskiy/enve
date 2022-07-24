use estring::SepVec;

const COMMA: char = ',';
const SEMI: char = ';';

pub type CommaVec<T> = SepVec<T, COMMA>;
pub type SemiVec<T> = SepVec<T, SEMI>;
