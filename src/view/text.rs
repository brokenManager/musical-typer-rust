use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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
  size: (u32, u32),
  align: TextAlign,
  texture: Texture<'ttf>,
}

impl<'ttf> Text<'ttf> {
  fn new<T>(
    builder: &TextBuilder<'ttf, T>,
  ) -> Result<Self, TextError> {
    let TextBuilder {
      font,
      texture_creator,
      text,
      align,
      color,
      line_height,
      ..
    } = builder;
    let text = if text == "" { " " } else { &text };
    let aspect = {
      let (x, y) =
        font.size_of(text).map_err(|e| TextError::FontError(e))?;
      x as f64 / y as f64
    };
    let surface = font
      .render(text)
      .blended(color.clone())
      .map_err(|e| TextError::FontError(e))?;
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| TextError::TextureError(e))?;
    Ok(Text {
      texture,
      size: ((aspect * *line_height as f64) as u32, *line_height),
      align: *align,
    })
  }

  pub fn render<T: RenderTarget>(
    &self,
    canvas: &mut Canvas<T>,
    pivot: Point,
  ) -> Result<(), TextError> {
    use TextAlign::*;
    let to = match self.align {
      Left => {
        Rect::new(pivot.x(), pivot.y(), self.size.0, self.size.1)
      }
      Center => Rect::from_center(pivot, self.size.0, self.size.1),
      Right => Rect::new(
        pivot.x() - self.size.0 as i32,
        pivot.y(),
        self.size.0,
        self.size.1,
      ),
    };
    canvas
      .copy(&self.texture, None, Some(to))
      .map_err(|e| TextError::RenderError(e))
  }

  pub fn render_with<T: RenderTarget, F>(
    &self,
    canvas: &mut Canvas<T>,
    f: F,
  ) -> Result<(), TextError>
  where
    F: FnOnce((u32, u32)) -> Point,
  {
    self.render(canvas, f(self.size))
  }
}

#[derive(Clone, Copy)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

pub struct TextBuilder<'a, T> {
  text: String,
  color: Color,
  line_height: u32,
  align: TextAlign,
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
      line_height: 20,
      align: TextAlign::Left,
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

  pub fn line_height(&mut self, new_line_height: u32) -> &mut Self {
    self.line_height = new_line_height;
    self
  }

  pub fn align(&mut self, new_align: TextAlign) -> &mut Self {
    self.align = new_align;
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
      let rendered = Text::new(self)?;
      self.cache.insert(String::from(self.cache_key()), rendered);
    }

    return Ok(self.cache.get(&key).unwrap());
  }
}
