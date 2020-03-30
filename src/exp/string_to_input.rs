use super::roman_lexer::RomanStr;

#[derive(Debug)]
pub struct StringToInput {
  origin: String,
  will_input: RomanStr,
  inputted: String,
}

impl StringToInput {
  pub fn new(
    origin: &str,
    to_input: &str,
  ) -> Result<StringToInput, String> {
    Ok(StringToInput {
      origin: origin.to_owned(),
      will_input: RomanStr::new(to_input)
        .map_err(|e| format!("{:?}", e))?,
      inputted: String::new(),
    })
  }

  pub fn origin(&self) -> &str {
    &self.origin
  }

  pub fn will_input(&self) -> String {
    let first_suggestions: Vec<&str> = self
      .will_input
      .exprs()
      .iter()
      .map(|options| options.standard_style())
      .collect();
    first_suggestions.join("").chars().collect()
  }

  pub fn inputted(&self) -> &str {
    self.inputted.as_str()
  }

  pub fn advance(self) -> StringToInput {
    self
  }
}
