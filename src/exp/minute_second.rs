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
  pub fn minutes(&self, minutes: u32) -> Self {
    MinuteSecond {
      minutes,
      seconds: self.seconds,
    }
  }
  pub fn seconds(&self, seconds: f64) -> Self {
    MinuteSecond {
      minutes: self.minutes,
      seconds,
    }
  }
  pub fn into_time(&self) -> f64 {
    self.minutes as f64 * 60.0 + self.seconds
  }
}
