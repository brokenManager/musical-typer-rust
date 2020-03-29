use crate::abst::controller::Controller;

pub struct SDLController {}

impl SDLController {}

impl Controller for SDLController {
  fn key_press(&mut self) -> char {
    todo!()
  }
  fn elapse_time(&mut self) -> f64 {
    todo!()
  }
}
