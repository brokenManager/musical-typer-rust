use super::roman::roman_lexer::RomanParseError;
use super::roman::roman_str::RomanStr;

pub struct TypingStr {
  pub will_input: String,
  pub inputted: String,
}

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

  pub fn new_with_inputted(
    origin: &str,
    to_input: &str,
    inputted: &str,
  ) -> Result<Self, RomanParseError> {
    let mut roman_str = RomanStr::new(to_input)?;
    for inputted in inputted.chars() {
      roman_str.input(inputted);
    }
    Ok(Sentence {
      origin: origin.to_owned(),
      hiragana: roman_str,
    })
  }

  pub fn empty() -> Self {
    Sentence {
      origin: "".to_owned(),
      hiragana: RomanStr::new("").unwrap(),
    }
  }

  pub fn from(origin: &str, yomigana: RomanStr) -> Self {
    Sentence {
      origin: origin.to_owned(),
      hiragana: yomigana,
    }
  }

  pub fn origin(&self) -> &str {
    self.origin.as_str()
  }

  pub fn yomiagana(&self) -> TypingStr {
    TypingStr {
      will_input: self.hiragana.will_input_yomigana().to_owned(),
      inputted: self.hiragana.inputted_yomigana(),
    }
  }

  pub fn roman(&self) -> TypingStr {
    TypingStr {
      will_input: self.hiragana.will_input_roman(),
      inputted: self.hiragana.inputted_roman().to_owned(),
    }
  }

  pub fn input(&mut self, typed: char) -> bool {
    self.hiragana.input(typed)
  }

  pub fn completed(&self) -> bool {
    self.hiragana.completed()
  }
}
