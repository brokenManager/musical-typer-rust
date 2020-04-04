use super::roman::roman_str::RomanStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Sentence {
  origin: String,
  hiragana: RomanStr,
}

impl Sentence {
  pub fn new(origin: &str, to_input: &str) -> Result<Self, String> {
    Ok(Sentence {
      origin: origin.to_owned(),
      hiragana: RomanStr::new(to_input)
        .map_err(|e| format!("{:#?}", e))?,
    })
  }

  pub fn origin(&self) -> &str {
    &self.origin
  }

  pub fn will_input(&self) -> String {
    let first_suggestions: Vec<&str> = self
      .hiragana
      .exprs()
      .iter()
      .map(|options| options.determined_style())
      .collect();
    first_suggestions.join("").chars().collect()
  }

  pub fn inputted(&self) -> &str {
    self.hiragana.inputted()
  }

  pub fn input(&mut self, c: char) {
    use super::roman::roman_char::RomanChar;

    if self.hiragana.exprs().iter().any(|predicate: &RomanChar| {
      predicate.styles().iter().any(|&style| {
        style
          .chars()
          .nth(0)
          .map_or(false, |predicate_char: char| predicate_char == c)
      })
    }) {}
  }

  pub fn ended_input(&self) -> bool {
    let roman_from_inputted = RomanStr::new(self.inputted());
    roman_from_inputted.map_or(
      false,
      |roman_from_inputted: RomanStr| {
        roman_from_inputted.exprs().len()
          == self.hiragana.exprs().len()
      },
    )
  }
}
