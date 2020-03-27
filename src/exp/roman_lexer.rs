use super::roman_char::{RomanChar, RomanParseError};

struct RomanLex(Vec<RomanChar>);

impl RomanLex {
  fn lex(hiragana: &str) -> Result<RomanLex, RomanParseError> {
    Err(RomanParseError::IllegalHiragana)
  }
}
