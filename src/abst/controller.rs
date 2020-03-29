pub trait Controller {
  fn key_press(&mut self) -> char;
  fn elapse_time(&mut self) -> f64;
}
