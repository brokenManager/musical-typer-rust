use super::ViewError;
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::{Canvas, Texture, TextureCreator},
  ttf::Font,
  video::{Window, WindowContext},
};
use std::{cell::RefCell, rc::Rc};

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

  pub fn paste_texture(
    &mut self,
    texture: &Texture,
    dst: Rect,
  ) -> Result<(), ViewError> {
    self
      .canvas
      .copy(texture, None, Some(dst))
      .map_err(|e| ViewError::RenderError(e))
  }
}
