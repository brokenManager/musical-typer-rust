use crate::model::exp::time::Seconds;

#[derive(Debug)]
enum ScoreEventKind {
  AC,
  WA,
  TLE,
}

#[derive(Debug)]
pub struct ScoreEvent {
  occured: Seconds,
  kind: ScoreEventKind,
}

impl ScoreEvent {
  fn new(occured: Seconds, kind: ScoreEventKind) -> Self {
    Self { occured, kind }
  }

  pub fn ac(time: Seconds) -> Self {
    Self::new(time, ScoreEventKind::AC)
  }

  pub fn wa(time: Seconds) -> Self {
    Self::new(time, ScoreEventKind::WA)
  }

  pub fn tle(time: Seconds) -> Self {
    Self::new(time, ScoreEventKind::TLE)
  }
}
