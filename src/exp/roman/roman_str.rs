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

  pub fn exprs(&self) -> &Vec<RomanChar> {
    &self.chars
  }

  pub fn inputted(&self) -> &str {
    &self.inputted
  }

  pub fn will_input(&self) -> String {
    self
      .chars
      .iter()
      .map(|roman_char: &RomanChar| {
        roman_char.determined_style().to_owned()
      })
      .collect::<Vec<String>>()
      .join("")[self.inputted().len()..]
      .to_owned()
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
}

#[test]
fn hello() {
  let mut hello = RomanStr::new("こんにちは").unwrap();
  assert_eq!(hello.inputted(), "");
  assert_eq!(hello.will_input(), "konnnitiha");
  assert!(hello.input('k'));
  assert_eq!(hello.inputted(), "k");
  assert_eq!(hello.will_input(), "onnnitiha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted(), "ko");
  assert_eq!(hello.will_input(), "nnnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted(), "kon");
  assert_eq!(hello.will_input(), "nnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted(), "konn");
  assert_eq!(hello.will_input(), "nitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted(), "konnn");
  assert_eq!(hello.will_input(), "itiha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted(), "konnni");
  assert_eq!(hello.will_input(), "tiha");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted(), "konnnit");
  assert_eq!(hello.will_input(), "iha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted(), "konnniti");
  assert_eq!(hello.will_input(), "ha");
  assert!(hello.input('h'));
  assert_eq!(hello.inputted(), "konnnitih");
  assert_eq!(hello.will_input(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted(), "konnnitiha");
  assert_eq!(hello.will_input(), "");
}

#[test]
fn toy() {
  let mut hello = RomanStr::new("おもちゃ").unwrap();
  assert_eq!(hello.inputted(), "");
  assert_eq!(hello.will_input(), "omocha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted(), "o");
  assert_eq!(hello.will_input(), "mocha");
  assert!(hello.input('m'));
  assert_eq!(hello.inputted(), "om");
  assert_eq!(hello.will_input(), "ocha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted(), "omo");
  assert_eq!(hello.will_input(), "cha");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted(), "omot");
  assert_eq!(hello.will_input(), "ya");
  assert!(hello.input('y'));
  assert_eq!(hello.inputted(), "omoty");
  assert_eq!(hello.will_input(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted(), "omotya");
  assert_eq!(hello.will_input(), "");
}
