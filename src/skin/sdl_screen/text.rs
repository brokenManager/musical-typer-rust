use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::ttf::Font;

pub struct Text<'ttf> {
  texture: Texture<'ttf>,
}

impl<'ttf> Text<'ttf> {
  fn new<T>(
    font: &'ttf Font<'ttf, 'static>,
    texture_creator: &'ttf TextureCreator<T>,
    text: &str,
    color: Color,
  ) -> Text<'ttf> {
    let surface = font.render(text).blended(color).unwrap();
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .unwrap();
    Text { texture }
  }

  pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, to: Rect) -> Result<(), String> {
    canvas.copy(&self.texture, None, Some(to))
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
  ) -> TextBuilder<'a, T> {
    TextBuilder {
      text: "".to_owned(),
      color: Color::RGB(0, 0, 0),
      font,
      texture_creator,
    }
  }

  pub fn text(&mut self, new_text: &str) -> &mut TextBuilder<'a, T> {
    self.text = new_text.to_owned();
    self
  }

  pub fn color(&mut self, new_color: Color) -> &mut TextBuilder<'a, T> {
    self.color = new_color;
    self
  }

  pub fn build(&self) -> Text<'a> {
    Text::new::<T>(
      self.font,
      self.texture_creator,
      self.text.as_str(),
      self.color,
    )
  }
}
