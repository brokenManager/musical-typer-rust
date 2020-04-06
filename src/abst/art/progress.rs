// Shows progress of music and stats

use super::{ARTNode, ART};

mod accuracy;
mod stats;

pub struct ProgressProps {
  completed: f64,
  correcntess: Option<f64>,
  accuracy: Option<f64>,
  welltyped_count: u32,
  mistyped_count: u32,
}

impl Default for ProgressProps {
  fn default() -> Self {
    ProgressProps {
      completed: 0.0,
      correcntess: None,
      accuracy: None,
      welltyped_count: 0,
      mistyped_count: 0,
    }
  }
}

pub struct Progress;

impl ARTNode<ProgressProps> for Progress {
  fn new(props: ProgressProps) -> Self {
    Progress
  }

  fn render(&self) -> ART {
    unimplemented!()
  }
}
