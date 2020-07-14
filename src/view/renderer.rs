use super::components::{Text, TextError, TextStyle};
use super::ViewError;
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::{Canvas, TextureCreator},
  ttf::Font,
  video::{Window, WindowContext},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type RenderCtx<'ttf, 'texture> =
  Rc<RefCell<Renderer<'ttf, 'texture>>>;

pub type ViewResult = Result<(), ViewError>;

pub trait Component {
  type Props;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool;

  fn update(&mut self, new_props: Self::Props);

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult;
}

pub struct Renderer<'ttf, 'texture> {
  width: u32,
  height: u32,
  canvas: Canvas<Window>,
  font: Font<'ttf, 'static>,
  text_cache: HashMap<TextStyle, Text<'texture>>,
  texture_creator: &'texture TextureCreator<WindowContext>,
}

impl<'ttf, 'texture> Renderer<'ttf, 'texture> {
  pub fn new(
    width: u32,
    height: u32,
    mut canvas: Canvas<Window>,
    font: Font<'ttf, 'static>,
    texture_creator: &'texture TextureCreator<WindowContext>,
  ) -> Result<Self, ViewError> {
    canvas.clear();
    canvas.present();

    Ok(Self {
      width,
      height,
      canvas,
      font,
      text_cache: HashMap::new(),
      texture_creator,
    })
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
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
