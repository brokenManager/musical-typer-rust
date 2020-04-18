use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::{
  render::Canvas,
  video::{Window, WindowContext},
};

use crate::view::text::{TextAlign, TextCtx, TextError};

pub struct HeaderProps {
  pub title: String,
  pub author: String,
  pub score_point: i32,
}

pub fn build(
  text_builder: TextCtx<'_, WindowContext>,
  props: HeaderProps,
) -> Result<
  impl Fn(&mut Canvas<Window>) -> Result<(), TextError> + '_,
  TextError,
> {
  let title_text = {
    text_builder
      .borrow_mut()
      .text(props.title.as_str())
      .color(Color::RGB(0, 0, 0))
      .line_height(50)
      .align(TextAlign::Right)
      .build()?
  };
  let author_text = {
    text_builder
      .borrow_mut()
      .text(props.author.as_str())
      .color(Color::RGB(156, 156, 162))
      .line_height(50)
      .align(TextAlign::Right)
      .build()?
  };
  let score_text = {
    text_builder
      .borrow_mut()
      .text(format!("{:08}", props.score_point).as_str())
      .color(Color::RGB(64, 79, 181))
      .line_height(50)
      .build()?
  };

  Ok(move |mut canvas: &mut Canvas<Window>| {
    title_text.render(&mut canvas, Point::new(800, 0))?;
    author_text.render(&mut canvas, Point::new(800, 50))?;
    score_text.render(&mut canvas, Point::new(0, 50))?;
    Ok(())
  })
}
