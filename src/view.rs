use crate::model::exp::scoremap::Scoremap;
use crate::model::game::MusicalTyperError;

mod gameview;
mod handler;
mod renderer;

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
use renderer::{text::TextError, RenderCtx};
use std::{cell::RefCell, rc::Rc};

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

pub struct Router<'ttf, 'canvas, 'sdl> {
  handler: Handler<'sdl>,
  renderer: RenderCtx<'ttf, 'canvas>,
}

impl<'ttf, 'canvas, 'sdl> Router<'ttf, 'canvas, 'sdl> {
  pub fn new(
    handler: Handler<'sdl>,
    renderer: Renderer<'ttf, 'canvas>,
  ) -> Self {
    Self {
      handler,
      renderer: Rc::new(RefCell::new(renderer)),
    }
  }

  pub fn run<'a: 'ttf + 'canvas + 'sdl>(
    &'a mut self,
    score: Scoremap,
  ) -> Result<(), ViewError> {
    let mut game_view = GameView::new(
      self.renderer.clone(),
      &mut self.handler,
      score,
      800,
      600,
    )?;
    game_view.run()?;

    Ok(())
  }
}

pub fn run_router(score: Scoremap) -> Result<(), ViewError> {
  let sdl = sdl2::init().unwrap();
  let ttf = sdl2::ttf::init().unwrap();
  sdl2::mixer::open_audio(
    44100,
    sdl2::mixer::DEFAULT_FORMAT,
    sdl2::mixer::DEFAULT_CHANNELS,
    1024,
  )
  .map_err(|e| ViewError::AudioError { message: e })?;

  let font = ttf
    .load_font(
      std::path::Path::new("./asset/mplus-1m-medium.ttf"),
      128,
    )
    .map_err(|e| ViewError::FontError {
      message: e.to_string(),
    })?;

  let video = sdl
    .video()
    .map_err(|e| ViewError::InitError { message: e })?;
  let window = video
    .window("Musical Typer", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| ViewError::InitError {
      message: e.to_string(),
    })?;

  let canvas = window.into_canvas().build().map_err(|e| {
    ViewError::InitError {
      message: e.to_string(),
    }
  })?;

  let handler = Handler::new(&sdl);
  let renderer = Renderer::new(canvas, font)?;

  Router::new(handler, renderer).run(score)?;
  Ok(())
}
