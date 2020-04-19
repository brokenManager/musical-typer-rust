use super::ViewError;
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::{Canvas, TextureCreator},
  ttf::{Font, Sdl2TtfContext},
  video::{Window, WindowContext},
  Sdl,
};
use std::collections::BTreeMap;
use text::{Text, TextStyle};

pub mod text;

pub struct Renderer<'ttf, 'canvas> {
  canvas: Canvas<Window>,
  texture_creator: TextureCreator<WindowContext>,
  font: Font<'ttf, 'static>,
  text_cache: BTreeMap<String, Text<'canvas>>,
}

impl<'ttf, 'canvas> Renderer<'ttf, 'canvas> {
  pub fn new(
    sdl: &Sdl,
    ttf: &'ttf Sdl2TtfContext,
    width: u32,
    height: u32,
  ) -> Result<Self, ViewError> {
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

    let texture_creator = canvas.texture_creator();

    Ok(Self {
      canvas,
      font,
      texture_creator,
      text_cache: BTreeMap::new(),
    })
  }

  pub fn flush(&mut self) {
    self.canvas.present();
  }

  pub fn clear(&mut self) {
    self.canvas.clear()
  }

  pub fn set_draw_color(&mut self, color: Color) {
    self.canvas.set_draw_color(color)
  }

  pub fn fill_rect(&mut self, rect: Rect) -> Result<(), ViewError> {
    self
      .canvas
      .fill_rect(rect)
      .map_err(|e| ViewError::RenderError(e))
  }

  pub fn draw_rect(&mut self, rect: Rect) -> Result<(), ViewError> {
    self
      .canvas
      .draw_rect(rect)
      .map_err(|e| ViewError::RenderError(e))
  }

  pub fn text<S>(&mut self, styler: S) -> Result<(), ViewError>
  where
    S: FnOnce(TextStyle) -> TextStyle,
  {
    let style = styler(TextStyle::new());
    let key = style.cache_key();

    if !self.text_cache.contains_key(&key) {
      self.text_cache.insert(
        key.clone(),
        Text::new(&style, &self.font, &self.texture_creator)?,
      );
    }

    let text =
      self.text_cache.get_mut(&key).ok_or(ViewError::CacheError)?;

    self
      .canvas
      .copy(text.texture(), None, Some(style.to_rect(text.aspect())))
      .map_err(|e| ViewError::RenderError(e))?;
    Ok(())
  }
}
