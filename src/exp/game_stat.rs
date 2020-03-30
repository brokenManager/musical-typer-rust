use super::note::Section;

pub struct GameActivity {
  typed_count: u32,
  mistyped_count: u32,
  typing_section_index: usize,
  sections: Vec<Section>,
}

impl GameActivity {
  pub fn new(sections: Vec<Section>) -> GameActivity {
    GameActivity {
      typed_count: 0,
      mistyped_count: 0,
      typing_section_index: 0,
      sections,
    }
  }

  pub fn accuracy(&self) -> f64 {
    self.mistyped_count as f64 / self.typed_count as f64
  }
}
