pub struct StringToInput {
  will_input: String,
  inputted: String,
}

impl StringToInput {
  pub fn new(toInput: &str) -> StringToInput {
    StringToInput {
      will_input: toInput.to_owned(),
      inputted: String::new(),
    }
  }

  pub fn will_input(&self) -> &str {
    self.will_input.as_str()
  }

  pub fn inputted(&self) -> &str {
    self.inputted.as_str()
  }

  pub fn advance(self) -> StringToInput {
    self
  }
}
