use super::roman_char::RomanChar;
use super::roman_lexer::{parse, RomanParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct RomanStr {
  chars: Vec<RomanChar>,
}

impl RomanStr {
  pub fn new(yomigana: &str) -> Result<Self, RomanParseError> {
    let chars: Vec<char> = yomigana.chars().collect();
    let mut parsed: Vec<RomanChar> = vec![];
    parse(&mut parsed, chars.as_slice())?;
    Ok(RomanStr { chars: parsed })
  }

  pub fn exprs(&self) -> &Vec<RomanChar> {
    &self.chars
  }

  pub fn inputted(&self) -> &str {
    ""
  }

  pub fn will_input(&self) -> &str {
    ""
  }

  pub fn input(&mut self, typed: char) {}
}

#[test]
fn hello() {
  let mut hello = RomanStr::new("こんにちは").unwrap();
  assert_eq!(hello.inputted(), "");
  assert_eq!(hello.will_input(), "konnnitiha");
  hello.input('k');
  assert_eq!(hello.inputted(), "k");
  assert_eq!(hello.will_input(), "onnnitiwa");
}
