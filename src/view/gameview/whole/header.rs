use sdl2::rect::Point;
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};
use crate::view::text::TextAlign;

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
    text_builder: &mut TextBuilder<'a, U>,
  ) -> Result<(), TextError> {
    use sdl2::pixels::Color;

    {
      let title_text = text_builder
        .text(self.title.as_str())
        .color(Color::RGB(0, 0, 0))
        .line_height(50)
        .align(TextAlign::Right)
        .build()?;
      title_text.render(&mut canvas, Point::new(800, 0))?;
    }

    {
      let author_text = text_builder
        .text(self.author.as_str())
        .color(Color::RGB(156, 156, 162))
        .line_height(50)
        .align(TextAlign::Right)
        .build()?;
      author_text.render(&mut canvas, Point::new(800, 50))?;
    }

    {
      let score_text = text_builder
        .text(format!("{:08}", self.score_point).as_str())
        .color(Color::RGB(64, 79, 181))
        .line_height(50)
        .build()?;

      score_text.render(&mut canvas, Point::new(0, 50))?;
    }

    Ok(())
  }
}
