use crate::model::{exp::scoremap::Scoremap, game::Presenter};
use crate::sdl::{SdlEventHandler, ViewError};

#[derive(Debug)]
pub enum MTError {
  ViewError(ViewError),
}

impl From<ViewError> for MTError {
  fn from(err: ViewError) -> Self {
    MTError::ViewError(err)
  }
}

pub struct MTController;

impl MTController {
  pub fn new() -> Self {
    MTController
  }

  pub fn run(&mut self, score: Scoremap) -> Result<(), MTError> {
    use crate::model::game::{MusicalTyper, MusicalTyperConfig};
    let mut game =
      MusicalTyper::new(score, self, MusicalTyperConfig::default());

    use crate::sdl::SdlView;
    let mut view = SdlView::new(800, 600, self)?;

    view.draw()?;
    Ok(())
  }
}

impl SdlEventHandler for MTController {
  fn key_press(&mut self, typed: impl Iterator<Item = char>) {}
  fn elapse_time(&mut self, delta_time: f64) {}
}

impl Presenter for MTController {
  fn play_bgm(&mut self, name: &std::primitive::str) {}
  fn decrease_remaining_time(
    &mut self,
    delta_time: std::primitive::f64,
  ) {
  }
  fn update_sentence(
    &mut self,
    string: &crate::model::exp::sentence::Sentence,
  ) {
  }
  fn pointed(&mut self, added: std::primitive::i32) {}
  fn typed(&mut self, is_mistaken: std::primitive::bool) {}
}
