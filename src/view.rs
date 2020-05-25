use crate::model::exp::{
  game_activity::GameScore, scoremap::Scoremap,
};
use crate::model::game::MusicalTyperError;
use game_view::GameView;
use handler::{HandleError, Handler};
use player::PlayerError;
use renderer::Renderer;
use renderer::{text::TextError, RenderCtx};
use result_view::ResultView;
use std::{cell::RefCell, rc::Rc};

mod game_view;
mod handler;
mod player;
mod renderer;
mod result_view;
mod stats;

#[derive(Debug)]
pub enum ViewError {
  ModelError(MusicalTyperError),
  InitError { message: String },
  FontError { message: String },
  PlayerError(PlayerError),
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

impl From<TextError> for ViewError {
  fn from(err: TextError) -> Self {
    ViewError::TextError(err)
  }
}

impl From<HandleError> for ViewError {
  fn from(err: HandleError) -> Self {
    ViewError::HandleError(err)
  }
}

pub trait View {
  fn run(&mut self) -> Result<(), ViewError>;
  fn next_route(&self) -> Option<ViewRoute>;
}

pub enum ViewRoute {
  GameView,
  ResultView(GameScore),
  Quit,
}

impl From<PlayerError> for ViewError {
  fn from(err: PlayerError) -> Self {
    ViewError::PlayerError(err)
  }
}

pub struct Router<'ttf, 'canvas> {
  handler: Handler,
  renderer: RenderCtx<'ttf, 'canvas>,
}

impl<'ttf, 'canvas> Router<'ttf, 'canvas> {
  pub fn new(
    handler: Handler,
    renderer: Renderer<'ttf, 'canvas>,
  ) -> Self {
    Self {
      handler,
      renderer: Rc::new(RefCell::new(renderer)),
    }
  }

  pub fn run(self, score: Scoremap) -> Result<(), ViewError> {
    let mut view: Option<Box<dyn View>> =
      Some(Box::new(ResultView::new(
        self.renderer.clone(),
        self.handler.clone(),
        GameScore::new(1000000, 0.5, 0.25),
      )));
    while let Some(boxed_view) = view.as_mut() {
      boxed_view.run()?;
      let next = boxed_view.next_route();
      match next {
        Some(ViewRoute::GameView) => {
          view.replace(Box::new(GameView::new(
            self.renderer.clone(),
            self.handler.clone(),
            score.clone(),
          )?));
        }
        Some(ViewRoute::ResultView(score)) => {
          view = Some(Box::new(ResultView::new(
            self.renderer.clone(),
            self.handler.clone(),
            score,
          )));
        }
        Some(ViewRoute::Quit) => {
          view = None;
        }
        _ => {}
      };
    }

    Ok(())
  }
}

pub fn run_router(score: Scoremap) -> Result<(), ViewError> {
  use std::path::Path;

  let sdl = sdl2::init().unwrap();
  let ttf = sdl2::ttf::init().unwrap();
  sdl2::mixer::open_audio(
    44100,
    sdl2::mixer::DEFAULT_FORMAT,
    sdl2::mixer::DEFAULT_CHANNELS,
    1024,
  )
  .map_err(|e| PlayerError::AudioError(e))?;
  sdl2::mixer::allocate_channels(32);

  let font = ttf
    .load_font(Path::new("./asset/mplus-1m-medium.ttf"), 128)
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
  let texture_creator = canvas.texture_creator();

  let handler = Handler::new(sdl);
  let renderer =
    Renderer::new(800, 600, canvas, font, &texture_creator)?;

  Router::new(handler, renderer).run(score)?;
  Ok(())
}
