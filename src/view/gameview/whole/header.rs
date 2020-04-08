use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};

pub struct Header {
  title: String,
  author: String,
  score_point: i32,
}

impl Header {
  pub fn new(title: &str, author: &str, score_point: i32) -> Self {
    Header {
      title: title.to_owned(),
      author: author.to_owned(),
      score_point,
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
  ) -> Result<(), TextError> {
    const JAPANESE_GLYPH_WIDTH: u32 = 13;
    use sdl2::pixels::Color;
    let title_text =
      text_builder.text(self.title.as_str()).build()?;
    let author_text = text_builder
      .text(self.author.as_str())
      .color(Color::RGB(156, 156, 162))
      .build()?;
    let score_text = text_builder
      .text(format!("{:08}", self.score_point).as_str())
      .color(Color::RGB(64, 79, 181))
      .build()?;
    let title_text_width =
      self.title.len() as u32 * JAPANESE_GLYPH_WIDTH;
    title_text.render(
      &mut canvas,
      Rect::new(
        800 - title_text_width as i32,
        0,
        title_text_width,
        50,
      ),
    )?;
    let author_text_width =
      self.author.len() as u32 * JAPANESE_GLYPH_WIDTH;
    author_text.render(
      &mut canvas,
      Rect::new(
        800 - author_text_width as i32,
        50,
        author_text_width,
        50,
      ),
    )?;
    score_text.render(&mut canvas, Rect::new(0, 50, 300, 50))?;
    Ok(())
  }
}
