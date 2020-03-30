#[derive(Debug, Clone)]
pub struct RomanChar {
  styles: Vec<&'static str>,
  determined: Option<&'static str>,
}

impl RomanChar {
  pub fn new(styles: &[&'static str]) -> RomanChar {
    RomanChar {
      styles: Vec::from(styles),
      determined: None,
    }
  }

  pub fn determine_from(&mut self, input: &str) {}

  pub fn standard_style(&self) -> &str {
    self.styles[0]
  }

  pub fn determined(&self) -> Option<&'static str> {
    self.determined
  }
}
