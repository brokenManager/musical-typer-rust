use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::ttf::Font;

#[derive(Debug)]
pub enum TextError {
  FontError(sdl2::ttf::FontError),
  TextureError(sdl2::render::TextureValueError),
  RenderError(String),
}

pub struct Text<'ttf> {
  texture: Texture<'ttf>,
}

impl<'ttf> Text<'ttf> {
  fn new<T>(
    font: &'ttf Font<'ttf, 'static>,
    texture_creator: &'ttf TextureCreator<T>,
    text: &str,
    color: Color,
  ) -> Result<Self, TextError> {
    let surface = font
      .render(text)
      .blended(color)
      .map_err(|e| TextError::FontError(e))?;
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| TextError::TextureError(e))?;
    Ok(Text { texture })
  }

  pub fn render<T: RenderTarget>(
    &self,
    canvas: &mut Canvas<T>,
    to: Rect,
  ) -> Result<(), TextError> {
    canvas
      .copy(&self.texture, None, Some(to))
      .map_err(|e| TextError::RenderError(e))
  }
}

pub struct TextBuilder<'a, T> {
  text: String,
  color: Color,
  font: &'a Font<'a, 'static>,
  texture_creator: &'a TextureCreator<T>,
}

impl<'a, T> Clone for TextBuilder<'a, T> {
  fn clone(&self) -> Self {
    TextBuilder {
      text: self.text.clone(),
      color: self.color,
      font: self.font,
      texture_creator: self.texture_creator,
    }
  }
}

impl<'a, T> TextBuilder<'a, T> {
  pub fn new(
    font: &'a Font<'a, 'static>,
    texture_creator: &'a TextureCreator<T>,
  ) -> Self {
    TextBuilder {
      text: "".to_owned(),
      color: Color::RGB(0, 0, 0),
      font,
      texture_creator,
    }
  }

  pub fn text(&mut self, new_text: &str) -> &mut Self {
    self.text = new_text.to_owned();
    self
  }

  pub fn color(&mut self, new_color: Color) -> &mut Self {
    self.color = new_color;
    self
  }

  pub fn build(&self) -> Result<Text<'a>, TextError> {
    let mut text = self.text.as_str();
    if text == "" {
      text = " ";
    }
    Text::new::<T>(self.font, self.texture_creator, text, self.color)
  }
}
