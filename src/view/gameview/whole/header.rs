use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};

pub struct Header {
  title: String,
  author: String,
}

impl Header {
  pub fn new(title: &str, author: &str) -> Self {
    Header {
      title: title.to_owned(),
      author: author.to_owned(),
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
  ) -> Result<(), TextError> {
    let title_text =
      text_builder.text(self.title.as_str()).build()?;
    let author_text =
      text_builder.text(self.author.as_str()).build()?;

    title_text.render(&mut canvas, Rect::new(0, 0, 300, 50))?;
    author_text.render(&mut canvas, Rect::new(0, 50, 300, 50))?;
    Ok(())
  }
}
