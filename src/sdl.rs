use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::time::Duration;

mod text;
mod whole;

use text::{TextBuilder, TextError};

#[derive(Debug)]
pub enum ViewError {
  InitError { message: String },
  FontError { message: String },
  TextError(TextError),
  RenderError(String),
}

impl From<TextError> for ViewError {
  fn from(err: TextError) -> Self {
    match err {
      TextError::RenderError(e) => ViewError::RenderError(e),
      _ => ViewError::TextError(err),
    }
  }
}

pub trait SdlEventHandler {
  fn key_press(&mut self, typed: Vec<char>);
  fn elapse_time(&mut self, delta_time: f64);
}

pub struct SdlView<'a, T> {
  width: u32,
  height: u32,
  ctx: Sdl,
  canvas: Canvas<Window>,
  controller: &'a mut T,
}

impl<'a, T> SdlView<'a, T>
where
  T: SdlEventHandler,
{
  pub fn new(
    width: u32,
    height: u32,
    controller: &'a mut T,
  ) -> Result<Self, ViewError> {
    let ctx = sdl2::init()
      .map_err(|e| ViewError::InitError { message: e })?;

    let video = ctx
      .video()
      .map_err(|e| ViewError::InitError { message: e })?;
    let window = video
      .window("Musical Typer", width, height)
      .position_centered()
      .opengl()
      .build()
      .map_err(|e| ViewError::InitError {
        message: e.to_string(),
      })?;

    let mut canvas = window.into_canvas().build().map_err(|e| {
      ViewError::InitError {
        message: e.to_string(),
      }
    })?;
    canvas.clear();
    canvas.present();

    Ok(SdlView {
      width,
      height,
      ctx,
      canvas,
      controller,
    })
  }

  pub fn draw(&mut self) -> Result<(), ViewError> {
    let texture_creator = self.canvas.texture_creator();

    let ttf =
      sdl2::ttf::init().map_err(|e| ViewError::InitError {
        message: e.to_string(),
      })?;
    let font = ttf
      .load_font(
        std::path::Path::new("./asset/mplus-1m-medium.ttf"),
        128,
      )
      .map_err(|e| ViewError::FontError {
        message: e.to_string(),
      })?;

    let builder = TextBuilder::new(&font, &texture_creator);

    'main: loop {
      let mut poller =
        self.ctx.event_pump().map_err(|e| ViewError::InitError {
          message: e.to_string(),
        })?;
      for event in poller.poll_iter() {
        use sdl2::event::Event::*;
        match event {
          Quit { .. } => break 'main,
          _ => {}
        }
        whole::render(
          &mut self.canvas,
          sdl2::rect::Rect::new(0, 0, self.width, self.height),
          builder.clone(),
        )?;

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
      }
    }
    Ok(())
  }
}
