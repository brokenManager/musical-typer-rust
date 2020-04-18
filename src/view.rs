use crate::model::exp::scoremap::Scoremap;
use crate::model::game::MusicalTyperError;

mod gameview;
mod text;

use gameview::GameView;
use text::TextError;

#[derive(Debug)]
pub enum ViewError {
  ModelError(MusicalTyperError),
  InitError { message: String },
  FontError { message: String },
  AudioError { message: String },
  TextError(TextError),
  RenderError(String),
}

impl From<MusicalTyperError> for ViewError {
  fn from(err: MusicalTyperError) -> Self {
    ViewError::ModelError(err)
  }
}

impl From<TextError> for ViewError {
  fn from(err: TextError) -> Self {
    match err {
      TextError::RenderError(e) => ViewError::RenderError(e),
      _ => ViewError::TextError(err),
    }
  }
}

pub struct Router {
  game_view: GameView,
}

impl Router {
  pub fn new(score: Scoremap) -> Result<Self, ViewError> {
    Ok(Router {
      game_view: GameView::new(800, 600, score)?,
    })
  }

  pub fn run(&mut self) -> Result<(), ViewError> {
    self.game_view.run()?;

    Ok(())
  }
}
