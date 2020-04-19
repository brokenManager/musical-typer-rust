use crate::model::exp::scoremap::Scoremap;
use crate::model::game::MusicalTyperError;

mod gameview;
mod handler;
mod renderer;

use gameview::GameView;
use handler::{HandleError, Handler};
use renderer::Renderer;
use sdl2::{ttf::Sdl2TtfContext, Sdl};

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
  handler: Handler<'sdl>,
  renderer: Renderer<'ttf, 'canvas>,
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
    sdl: &'sdl Sdl,
    ttf: &'ttf Sdl2TtfContext,
    score: Scoremap,
  ) -> Result<Self, ViewError> {
    let handler = Handler::new(&sdl);
    let renderer = Renderer::new(&sdl, &ttf, 800, 600)?;
    Ok(Router {
      renderer,
      handler,
      game_view: GameView::new(&renderer, &handler, score, 800, 600)?,
    })
  }

  pub fn run(&mut self) -> Result<(), ViewError> {
    self.game_view.run()?;

    Ok(())
  }
}
