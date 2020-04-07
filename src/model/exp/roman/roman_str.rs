use super::roman_char::RomanChar;
use super::roman_lexer::{parse, RomanParseError};

#[derive(Clone, PartialEq)]
pub struct RomanStr {
  chars: Vec<RomanChar>,
  inputting_char: usize,
  inputted: String,
}

impl std::fmt::Debug for RomanStr {
  fn fmt(
    &self,
    mut f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      &mut f,
      "{:?}",
      self
        .chars
        .iter()
        .map(|c: &RomanChar| format!("{:?}", c))
        .collect::<Vec<String>>()
        .join(" ")
    )
  }
}

impl RomanStr {
  pub fn new(yomigana: &str) -> Result<Self, RomanParseError> {
    let chars: Vec<char> = yomigana.chars().collect();
    let mut parsed: Vec<RomanChar> = vec![];
    parse(&mut parsed, chars.as_slice())?;
    Ok(RomanStr {
      chars: parsed,
      inputting_char: 0,
      inputted: String::new(),
    })
  }

  pub fn will_input_yomigana(&self) -> &str {
    ""
  }

  pub fn inputted_yomigana(&self) -> String {
    self
      .chars
      .iter()
      .take_while(|c| !c.completed_input())
      .map(|c| c.origin())
      .collect::<Vec<&str>>()
      .join("")
      .to_owned()
  }

  pub fn will_input_roman(&self) -> String {
    self
      .chars
      .iter()
      .map(|roman_char: &RomanChar| {
        roman_char.determined_style().to_owned()
      })
      .collect::<Vec<String>>()
      .join("")[self.inputted_roman().len()..]
      .to_owned()
  }

  pub fn inputted_roman(&self) -> &str {
    &self.inputted
  }

  pub fn input(&mut self, typed: char) -> bool {
    if self.chars[self.inputting_char].input(typed) {
      self.inputted.push(typed);
      if self.chars[self.inputting_char].completed_input() {
        self.inputting_char += 1;
      }
      true
    } else {
      false
    }
  }

  pub fn completed(&self) -> bool {
    self.chars.len() <= self.inputting_char
  }
}

#[test]
fn hello() -> Result<(), RomanParseError> {
  let mut hello = RomanStr::new("こんにちは")?;
  assert_eq!(hello.inputted_roman(), "");
  assert_eq!(hello.will_input_roman(), "konnnitiha");
  assert!(hello.input('k'));
  assert_eq!(hello.inputted_roman(), "k");
  assert_eq!(hello.will_input_roman(), "onnnitiha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "ko");
  assert_eq!(hello.will_input_roman(), "nnnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "kon");
  assert_eq!(hello.will_input_roman(), "nnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "konn");
  assert_eq!(hello.will_input_roman(), "nitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "konnn");
  assert_eq!(hello.will_input_roman(), "itiha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "konnni");
  assert_eq!(hello.will_input_roman(), "tiha");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "konnnit");
  assert_eq!(hello.will_input_roman(), "iha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "konnniti");
  assert_eq!(hello.will_input_roman(), "ha");
  assert!(hello.input('h'));
  assert_eq!(hello.inputted_roman(), "konnnitih");
  assert_eq!(hello.will_input_roman(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted_roman(), "konnnitiha");
  assert_eq!(hello.will_input_roman(), "");
  Ok(())
}

#[test]
fn toy() -> Result<(), RomanParseError> {
  let mut hello = RomanStr::new("おもちゃ")?;
  assert_eq!(hello.inputted_roman(), "");
  assert_eq!(hello.will_input_roman(), "omocha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "o");
  assert_eq!(hello.will_input_roman(), "mocha");
  assert!(hello.input('m'));
  assert_eq!(hello.inputted_roman(), "om");
  assert_eq!(hello.will_input_roman(), "ocha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "omo");
  assert_eq!(hello.will_input_roman(), "cha");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "omot");
  assert_eq!(hello.will_input_roman(), "ya");
  assert!(hello.input('y'));
  assert_eq!(hello.inputted_roman(), "omoty");
  assert_eq!(hello.will_input_roman(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted_roman(), "omotya");
  assert_eq!(hello.will_input_roman(), "");
  Ok(())
}
