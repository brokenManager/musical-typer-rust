use super::ViewError;
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::{Canvas, TextureCreator},
  ttf::Font,
  video::{Window, WindowContext},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use text::{Text, TextError, TextStyle};

pub mod text;

pub type RenderCtx<'ttf, 'texture> =
  Rc<RefCell<Renderer<'ttf, 'texture>>>;

pub type ViewResult = Result<(), ViewError>;

pub struct Renderer<'ttf, 'texture> {
  canvas: Canvas<Window>,
  font: Font<'ttf, 'static>,
  text_cache: HashMap<TextStyle, Text<'texture>>,
  texture_creator: &'texture TextureCreator<WindowContext>,
}

impl<'ttf, 'texture> Renderer<'ttf, 'texture> {
  pub fn new(
    mut canvas: Canvas<Window>,
    font: Font<'ttf, 'static>,
    texture_creator: &'texture TextureCreator<WindowContext>,
  ) -> Result<Self, ViewError> {
    canvas.clear();
    canvas.present();

    Ok(Self {
      canvas,
      font,
      text_cache: HashMap::new(),
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

    if !self.text_cache.contains_key(&style) {
      let text = Text::new(style.clone(), &self.font, |surface| {
        self
          .texture_creator
          .create_texture_from_surface(surface)
          .map_err(|e| TextError::TextureError(e))
      })?;
      self.text_cache.insert(style.clone(), text);
    }

    let text = self
      .text_cache
      .get_mut(&style)
      .ok_or(ViewError::CacheError)?;

    text.render(&mut self.canvas)?;
    Ok(())
  }
}
