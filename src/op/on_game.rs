use crate::exp::string_to_input::StringToInput;

pub trait Controller {
  fn key_press(&mut self) -> char;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_string_to_input(&mut self, string: &StringToInput);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}

pub fn run_game(
  controller: &mut impl Controller,
  presenter: &mut impl Presenter,
) -> Result<(), String> {
  Ok(())
}
