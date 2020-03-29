use crate::abst::controller::Controller;
use crate::abst::presenter::Presenter;

pub fn run_game(
  controller: &mut impl Controller,
  presenter: &mut impl Presenter,
) -> Result<(), String> {
  Ok(())
}
