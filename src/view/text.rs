use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::ttf::Font;
use std::collections::HashMap;

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
  cache: HashMap<String, Text<'a>>,
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
      cache: HashMap::new(),
    }
  }

  pub fn text(&mut self, new_text: &str) -> &mut Self {
    if new_text == "" {
      self.text = String::from(" ");
    } else {
      self.text = new_text.to_owned();
    }

    self
  }

  pub fn color(&mut self, new_color: Color) -> &mut Self {
    self.color = new_color;
    self
  }

  fn cache_key(&self) -> String {
    format!(
      "{},{},{},{},{},",
      self.text,
      self.color.r,
      self.color.g,
      self.color.b,
      self.color.a
    )
  }

  pub fn build(&mut self) -> Result<&Text<'a>, TextError> {
    let key = self.cache_key();
    if !self.cache.contains_key(&key) {
      let rendered = Text::new::<T>(
        self.font,
        self.texture_creator,
        self.text.as_str(),
        self.color,
      )?;
      self.cache.insert(String::from(self.cache_key()), rendered);
    }

    return Ok(self.cache.get(&key).unwrap());
  }
}
