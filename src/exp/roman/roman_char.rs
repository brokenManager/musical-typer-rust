#[derive(Debug, Clone, PartialEq)]
pub struct RomanChar {
  styles: Vec<&'static str>,
  determined_style: Option<&'static str>,
  inputted: String,
}

impl RomanChar {
  pub fn new(styles: &[&'static str]) -> Self {
    RomanChar {
      styles: Vec::from(styles),
      determined_style: None,
      inputted: String::new(),
    }
  }

  fn determine(&mut self, input: &str) -> Option<&'static str> {
    let filtered: Vec<_> = self
      .styles
      .iter()
      .filter(|style| style.starts_with(input))
      .collect();
    if filtered.len() < 1 {
      None
    } else {
      Some(filtered[0])
    }
  }

  fn standard_style(&self) -> &str {
    self.styles[0]
  }

  pub fn styles(&self) -> &[&str] {
    &self.styles
  }

  pub fn determined_style(&self) -> &str {
    self.determined_style.unwrap_or(self.styles[0])
  }

  pub fn input(&mut self, typed: char) {
    let to_test = [self.inputted.clone(), typed.to_string()].concat();
    if let Some(determined) = self.determine(&to_test) {
      self.determined_style = Some(determined);
      self.inputted = to_test;
    } else {
      self.determined_style = None;
    }
  }
}

#[test]
fn tea() {
  let mut tea = RomanChar::new(&["cha", "cya", "tya"]);
  assert_eq!("cha", tea.determined_style());
  tea.input('c');
  assert_eq!("cha", tea.determined_style());
  tea.input('y');
  assert_eq!("cya", tea.determined_style());

  let mut tea = RomanChar::new(&["cha", "cya", "tya"]);
  assert_eq!("cha", tea.determined_style());
  tea.input('t');
  assert_eq!("tya", tea.determined_style());
}
