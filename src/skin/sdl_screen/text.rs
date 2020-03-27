use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub struct Text<'ttf> {
  texture: Texture<'ttf>,
}

impl<'ttf> Text<'ttf> {
  fn new(
    font: &'ttf Font<'ttf, 'static>,
    texture_creator: &'ttf TextureCreator<WindowContext>,
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

pub struct TextBuilder {
  text: String,
  color: Color,
}

impl TextBuilder {
  pub fn new<'a>() -> TextBuilder {
    TextBuilder {
      text: "".to_owned(),
      color: Color::RGB(0, 0, 0),
    }
  }

  pub fn text(&mut self, new_text: &str) -> &mut TextBuilder {
    self.text = new_text.to_owned();
    self
  }

  pub fn color(&mut self, new_color: Color) -> &mut TextBuilder {
    self.color = new_color;
    self
  }

  pub fn build<'a>(
    &self,
    font: &'a Font<'a, 'static>,
    texture_creator: &'a TextureCreator<WindowContext>,
  ) -> Text<'a> {
    Text::new(font, texture_creator, self.text.as_str(), self.color)
  }
}
