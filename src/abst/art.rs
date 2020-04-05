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
  Label,
  Box,
  Fragment,
}

pub trait ARTNode<P>
where
  P: Default,
{
  fn new(props: P) -> Self;

  type T: IntoIterator<Item = ART>;
  fn render(&self) -> Self::T;
}

pub enum ART {
  Node(Box<ART>),
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

pub struct ARTRenderer {
  tree: ART,
}

impl ARTRenderer {
  fn new() -> Self {
    ARTRenderer {
      tree: ART::End(ARTEndNode::Fragment),
    }
  }

  fn tree(&self) -> &ART {
    &self.tree
  }
}
