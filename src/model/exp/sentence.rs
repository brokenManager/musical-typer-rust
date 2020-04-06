use super::roman::roman_lexer::RomanParseError;
use super::roman::roman_str::RomanStr;

#[derive(Clone, PartialEq)]
pub struct Sentence {
  origin: String,
  hiragana: RomanStr,
}

impl std::fmt::Debug for Sentence {
  fn fmt(
    &self,
    mut f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(&mut f, "{} <-> {:?}", self.origin, self.hiragana)
  }
}

impl Sentence {
  pub fn new(
    origin: &str,
    to_input: &str,
  ) -> Result<Self, RomanParseError> {
    Ok(Sentence {
      origin: origin.to_owned(),
      hiragana: RomanStr::new(to_input)?,
    })
  }

  pub fn from(origin: &str, yomigana: RomanStr) -> Self {
    Sentence {
      origin: origin.to_owned(),
      hiragana: yomigana,
    }
  }

  pub fn origin(&self) -> &str {
    &self.origin
  }

  pub fn hiragana(&self) -> &RomanStr {
    &self.hiragana
  }

  pub fn input(&mut self, typed: char) -> bool {
    self.hiragana.input(typed)
  }
}
