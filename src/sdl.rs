extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use std::time::Duration;

use crate::model::exp::sentence::Sentence;

mod header;
mod keyboard;
mod section;
mod text;

use header::Header;
use keyboard::Keyboard;
use section::Section;
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
  fn quit(&mut self);
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

  fn render<'t>(
    &mut self,
    builder: TextBuilder<'t, WindowContext>,
  ) -> Result<(), ViewError> {
    let header = Header::new("Music Name", "Composer");
    let header_dim = Rect::new(0, 0, self.width, 100);

    let to_input =
      Sentence::new("千本桜　夜ニ紛レ", "せんぼんざくらよるにまぎれ")
        .map_err(|e| ViewError::InitError {
          message: format!("{:?}", e),
        })?;
    let section = Section::new(&to_input, 0.2);
    let section_dim = Rect::new(0, 100, self.width, 200);

    let keyboard = Keyboard::new(&['h']);
    let keyboard_dim =
      Rect::new(0, self.height as i32 - 300, self.width, 300);

    self.canvas.set_draw_color(Color::RGB(253, 243, 226));
    self.canvas.clear();

    header.draw(&mut self.canvas, builder.clone())?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self
      .canvas
      .draw_rect(header_dim)
      .map_err(|e| ViewError::RenderError(e))?;

    section.draw(&mut self.canvas, builder.clone(), section_dim)?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self
      .canvas
      .draw_rect(section_dim)
      .map_err(|e| ViewError::RenderError(e))?;

    keyboard.draw(&mut self.canvas, builder.clone(), keyboard_dim)?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self
      .canvas
      .draw_rect(keyboard_dim)
      .map_err(|e| ViewError::RenderError(e))?;
    Ok(())
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

    let mut poller =
      self.ctx.event_pump().map_err(|e| ViewError::InitError {
        message: e.to_string(),
      })?;
    for event in poller.poll_iter() {
      use sdl2::event::Event::*;
      match event {
        Quit { .. } => self.controller.quit(),
        _ => {}
      }
      self.render(builder.clone())?;

      self.canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
  }
}
