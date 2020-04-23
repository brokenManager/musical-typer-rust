#[derive(Debug, Clone)]
pub struct AchievementRate {
  correction_type_count: u32,
  whole_type_count: u32,
}

impl AchievementRate {
  pub fn new() -> Self {
    Self {
      correction_type_count: 0,
      whole_type_count: 0,
    }
  }
}
