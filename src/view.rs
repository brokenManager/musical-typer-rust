use crate::model::exp::scoremap::Scoremap;
use crate::model::game::MusicalTyperError;

mod gameview;
pub mod handler;
pub mod renderer;

use gameview::GameView;
use handler::{HandleError, Handler};
use renderer::Renderer;

#[derive(Debug)]
pub enum ViewError {
  ModelError(MusicalTyperError),
  InitError { message: String },
  FontError { message: String },
  AudioError { message: String },
  TextError(TextError),
  RenderError(String),
  CacheError,
  HandleError(HandleError),
}

impl From<MusicalTyperError> for ViewError {
  fn from(err: MusicalTyperError) -> Self {
    ViewError::ModelError(err)
  }
}
use renderer::text::TextError;

impl From<TextError> for ViewError {
  fn from(err: TextError) -> Self {
    match err {
      TextError::RenderError(e) => ViewError::RenderError(e),
      _ => ViewError::TextError(err),
    }
  }
}

impl From<HandleError> for ViewError {
  fn from(err: HandleError) -> Self {
    ViewError::HandleError(err)
  }
}

pub struct Router<'renderer, 'ttf, 'canvas, 'handler, 'sdl> {
  game_view: GameView<'renderer, 'ttf, 'canvas, 'handler, 'sdl>,
}

impl<'renderer, 'ttf, 'canvas, 'handler, 'sdl>
  Router<'renderer, 'ttf, 'canvas, 'handler, 'sdl>
where
  'sdl: 'handler,
  'ttf: 'renderer,
  'canvas: 'renderer,
{
  pub fn new(
    handler: &'handler mut Handler<'sdl>,
    renderer: &'renderer mut Renderer<'ttf, 'canvas>,
    score: Scoremap,
  ) -> Result<Self, ViewError> {
    Ok(Self {
      game_view: GameView::new(renderer, handler, score, 800, 600)?,
    })
  }

  pub fn run<'a: 'renderer + 'ttf + 'canvas + 'handler + 'sdl>(
    &'a mut self,
  ) -> Result<(), ViewError> {
    self.game_view.run()?;

    Ok(())
  }
}
