use super::super::note::TypeResult;

#[derive(Debug, Default, Clone)]
pub struct Accuracy {
  correction_type_count: u32,
  wrong_type_count: u32,
}

impl Accuracy {
  fn point(&mut self, res: &TypeResult) {
    if let TypeResult::Succeed = res {
      self.correction_type_count += 1;
    } else if let TypeResult::Mistaken = res {
      self.wrong_type_count += 1;
    }
  }

  pub fn as_f64(&self) -> f64 {
    if self.correction_type_count + self.wrong_type_count == 0 {
      return 0.0;
    }
    self.correction_type_count as f64
      / (self.correction_type_count + self.wrong_type_count) as f64
  }
}

impl From<Accuracy> for f64 {
  fn from(acc: Accuracy) -> Self {
    acc.as_f64()
  }
}

#[derive(Debug, Default, Clone)]
pub struct AchievementRate {
  correction_type_count: u32,
  whole_type_count: u32,
}

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
