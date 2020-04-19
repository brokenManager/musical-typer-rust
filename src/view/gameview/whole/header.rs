use crate::view::{
  renderer::{text::TextAlign, Renderer},
  ViewError,
};
use sdl2::rect::Point;

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

  pub fn draw<'texture>(
    &self,
    mut canvas: &'texture mut Renderer<'_, 'texture>,
  ) -> Result<(), ViewError> {
    use sdl2::pixels::Color;

    let title_text = canvas.text(|s| {
      s.text(self.title.as_str())
        .color(Color::RGB(0, 0, 0))
        .line_height(50)
        .align(TextAlign::Right)
        .pos(Point::new(800, 0))
    })?;

    let author_text = canvas.text(|s| {
      s.text(self.author.as_str())
        .color(Color::RGB(156, 156, 162))
        .line_height(50)
        .align(TextAlign::Right)
        .pos(Point::new(800, 50))
    })?;

    let score_text = canvas.text(|s| {
      s.text(format!("{:08}", self.score_point).as_str())
        .color(Color::RGB(64, 79, 181))
        .line_height(50)
        .pos(Point::new(0, 50))
    })?;

    Ok(())
  }
}
