use super::ViewError;
use sdl2::{
  pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window,
};
use std::collections::BTreeMap;
use text::{Text, TextStyle};

pub mod text;

pub struct Renderer<'ttf, 'canvas> {
  canvas: &'canvas mut Canvas<Window>,
  font: Font<'ttf, 'static>,
  text_cache: BTreeMap<String, Text<'canvas>>,
}

impl<'ttf, 'canvas> Renderer<'ttf, 'canvas> {
  pub fn new(
    canvas: &'canvas mut Canvas<Window>,
    font: Font<'ttf, 'static>,
    width: u32,
    height: u32,
  ) -> Result<Self, ViewError> {
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    Ok(Self {
      canvas,
      font,
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
      let text = Text::new(
        &style,
        &self.font,
        &self.canvas.texture_creator(),
      )?;
      self.text_cache.insert(key.clone(), text);
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
