use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{
  render::Canvas,
  video::{Window, WindowContext},
};

use super::super::super::text::{TextBuilder, TextError};

pub struct HeaderProps {
  pub title: String,
  pub author: String,
  pub score_point: i32,
}

pub fn build(
  mut text_builder: TextBuilder<'_, WindowContext>,
  props: HeaderProps,
) -> Result<
  impl Fn(&mut Canvas<Window>) -> Result<(), TextError> + '_,
  TextError,
> {
  const JAPANESE_GLYPH_WIDTH: u32 = 13;
  let title_text = text_builder.text(props.title.as_str()).build()?;
  let author_text = text_builder
    .text(props.author.as_str())
    .color(Color::RGB(156, 156, 162))
    .build()?;
  let score_text = text_builder
    .text(format!("{:08}", props.score_point).as_str())
    .color(Color::RGB(64, 79, 181))
    .build()?;
  let title_text_width =
    props.title.len() as u32 * JAPANESE_GLYPH_WIDTH;
  let author_text_width =
    props.author.len() as u32 * JAPANESE_GLYPH_WIDTH;

  Ok(move |mut canvas: &mut Canvas<Window>| {
    title_text.render(
      &mut canvas,
      Rect::new(
        800 - title_text_width as i32,
        0,
        title_text_width,
        50,
      ),
    )?;
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
  })
}
