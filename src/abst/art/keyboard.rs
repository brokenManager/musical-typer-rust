// Shows typed and pointed animation on cells of key

use super::{ARTNode, ART};

mod key_cell;

pub struct PointEvent {
  amount: u32,
}

pub struct KeyboardProps {
  highlighted_keys: Vec<char>,
  point_event: Option<PointEvent>,
}

impl Default for KeyboardProps {
  fn default() -> Self {
    KeyboardProps {
      highlighted_keys: vec![],
      point_event: None,
    }
  }
}

pub struct Keyboard;

impl ARTNode<KeyboardProps> for Keyboard {
  fn new(props: KeyboardProps) -> Self {
    Keyboard
  }

  fn render(&self) -> ART {
    unimplemented!()
  }
}
