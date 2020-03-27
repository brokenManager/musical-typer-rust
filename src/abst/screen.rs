use crate::exp::string_to_input::StringToInput;

pub trait Screen {
  fn play_bgm(name: &str);
  fn decrease_remaining_time(delta_time: f64);
  fn update_string_to_input(string: &StringToInput);
}
