use super::ViewError;
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::{Canvas, TextureCreator},
  ttf::Font,
  video::{Window, WindowContext},
};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use text::{Text, TextError, TextStyle};

pub mod text;

pub type RenderCtx<'ttf, 'canvas> =
  Rc<RefCell<Renderer<'ttf, 'canvas>>>;

pub struct Renderer<'ttf, 'canvas> {
  canvas: Canvas<Window>,
  font: Font<'ttf, 'static>,
  text_cache: BTreeMap<String, Text<'canvas>>,
  texture_creator: Rc<TextureCreator<WindowContext>>,
}

impl<'ttf, 'canvas> Renderer<'ttf, 'canvas> {
  pub fn new(
    mut canvas: Canvas<Window>,
    font: Font<'ttf, 'static>,
  ) -> Result<Self, ViewError> {
    canvas.clear();
    canvas.present();

    let texture_creator = Rc::new(canvas.texture_creator());

    Ok(Self {
      canvas,
      font,
      text_cache: BTreeMap::new(),
      texture_creator,
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
      let text = Text::new(&style, &self.font)?;
      self.text_cache.insert(key.clone(), text);
    }

    let text =
      self.text_cache.get_mut(&key).ok_or(ViewError::CacheError)?;

    let texture = self
      .texture_creator
      .create_texture_from_surface(&text.surface())
      .map_err(|e| TextError::TextureError(e))?;

    self
      .canvas
      .copy(&texture, None, Some(style.to_rect(text.aspect())))
      .map_err(|e| ViewError::RenderError(e))?;
    Ok(())
  }
}
