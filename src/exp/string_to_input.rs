use super::roman_lexer::RomanStr;

pub struct StringToInput {
  will_input: RomanStr,
  inputted: String,
}

impl StringToInput {
  pub fn new(to_input: &str) -> Result<StringToInput, String> {
    Ok(StringToInput {
      will_input: RomanStr::new(to_input)
        .map_err(|e| format!("{:?}", e))?,
      inputted: String::new(),
    })
  }

  pub fn will_input(&self) -> String {
    let first_suggestions: Vec<&str> = self
      .will_input
      .exprs()
      .iter()
      .map(|options| options[0])
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
