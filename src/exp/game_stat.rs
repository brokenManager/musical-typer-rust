use super::note::Section;

pub struct GameActivity {
  typed_count: u32,
  mistyped_count: u32,
  typing_section_index: usize,
  sections: Vec<Section>,
}

impl GameActivity {
  pub fn new(sections: Vec<Section>) -> Self {
    GameActivity {
      typed_count: 0,
      mistyped_count: 0,
      typing_section_index: 0,
      sections,
    }
  }

  pub fn current_section(&self) -> Option<Section> {
    if self.typing_section_index < self.sections.len() {
      Some(self.sections[self.typing_section_index].clone())
    } else {
      None
    }
  }

  pub fn accuracy(&self) -> f64 {
    self.mistyped_count as f64 / self.typed_count as f64
  }
}
