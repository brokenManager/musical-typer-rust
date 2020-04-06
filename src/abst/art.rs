use super::prim::{Area, Color, Pos};

mod components;
mod finder;
mod header;
mod keyboard;
mod progress;

pub enum RenderError {
  FontError(String),
  TextureError(String),
  RenderError(String),
}

pub enum ARTEndNode {
  Label {
    pos: Pos,
    text: String,
    color: Color,
  },
  Box {
    area: Area,
    border_color: Color,
    background_color: Color,
  },
  Fragment,
}

pub trait ARTNode<P>
where
  P: Default,
{
  fn new(props: P) -> Self;

  fn render(&self) -> ART;
}

pub enum ART {
  Node {
    client: Area,
    children: Vec<Box<ART>>,
  },
  End(ARTEndNode),
}

/*
ART
L Header
| L MusicInfo
| | L Title
| | L Author
| | L Singer
| L PlayerInfo
| | L Point
| |   L Animated
| L DebugInfo
L Finder
| L Japanese
| | L Inputted
| | L WillInput
| L Yomigana
| | L Inputted
| | L WillInput
| L Roman
| | L Inputted
| | L WillInput
| L Combo
|   L Gauge
|   L Count
L Keyboard
| L KeyCell
|   L Animated
L Progress
  L Accuracy
  | L Gauge
  L Stats
    L Correctness
    L Completed
*/
