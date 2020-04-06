// Shows information about music and system

use super::{ARTNode, ART};

mod debug_info;
mod music_info;
mod player_info;

pub enum PointEvent {
  AC,
  WA,
  TLE,
}

pub struct HeaderProps {
  fps: f64,
  song_name: String,
  author_name: String,
  score: u32,
  point_event: Option<PointEvent>,
}

impl Default for HeaderProps {
  fn default() -> Self {
    HeaderProps {
      fps: 0.0,
      song_name: "".to_owned(),
      author_name: "".to_owned(),
      score: 0,
      point_event: None,
    }
  }
}

pub struct Header;

impl ARTNode<HeaderProps> for Header {
  fn new(props: HeaderProps) -> Self {
    Header
  }

  fn render(&self) -> ART {
    unimplemented!()
  }
}
