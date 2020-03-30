#[derive(Debug, Clone)]
pub struct RomanChar {
  styles: Vec<&'static str>,
}

impl RomanChar {
  pub fn new(styles: &[&'static str]) -> RomanChar {
    RomanChar {
      styles: Vec::from(styles),
    }
  }

  pub fn determine_from(
    &mut self,
    input: &str,
  ) -> Option<&'static str> {
    let filtered: Vec<_> = self
      .styles
      .iter()
      .filter(|style| style.starts_with(input))
      .collect();
    if filtered.len() != 1 {
      None
    } else {
      Some(filtered[0])
    }
  }

  pub fn standard_style(&self) -> &str {
    self.styles[0]
  }
}
