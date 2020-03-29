use crate::exp::string_to_input::StringToInput;

pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_string_to_input(&mut self, string: &StringToInput);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}
