pub struct MinuteSecond {
  minutes: u32,
  seconds: f64,
}

impl MinuteSecond {
  pub fn new() -> Self {
    MinuteSecond {
      minutes: 0,
      seconds: 0.0,
    }
  }
  pub fn minutes(&mut self, minutes: u32) {
    self.minutes = minutes;
  }
  pub fn seconds(&mut self, seconds: f64) {
    self.seconds = seconds;
  }
  pub fn into_time(&self) -> f64 {
    self.minutes as f64 * 60.0 + self.seconds
  }
}
