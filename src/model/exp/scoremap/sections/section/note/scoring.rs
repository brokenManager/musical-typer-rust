use super::super::note::TypeResult;

mod accuracy;
mod achievement_rate;

use accuracy::Accuracy;
use achievement_rate::AchievementRate;

#[derive(Debug, Default, Clone)]
pub struct Scoring {
  accuracy: Accuracy,
  achievement_rate: AchievementRate,
}

impl Scoring {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn point(&mut self, res: &TypeResult) {
    self.accuracy.point(res);
  }

  pub fn accuracy(&self) -> &Accuracy {
    &self.accuracy
  }
}
