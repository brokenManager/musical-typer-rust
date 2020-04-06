// Shows text to type and combos

use super::{ARTNode, ART};
use crate::exp::sentence::Sentence;

mod combo;
mod japanese;
mod roman;
mod yomigana;

pub struct FinderProps {
  combo_count: u32,
  sentence: Option<Sentence>,
}

impl Default for FinderProps {
  fn default() -> Self {
    FinderProps {
      combo_count: 0,
      sentence: None,
    }
  }
}

pub struct Finder;

impl ARTNode<FinderProps> for Finder {
  fn new(props: FinderProps) -> Self {
    Finder
  }

  fn render(&self) -> ART {
    unimplemented!()
  }
}
