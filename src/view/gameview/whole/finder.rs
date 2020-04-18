use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{
  render::Canvas,
  video::{Window, WindowContext},
};

use super::super::super::text::{TextCtx, TextError};

use crate::model::exp::sentence::Sentence;

mod sentence;

pub struct FinderProps {
  pub sentence: Option<Sentence>,
  pub remaining_ratio: f64,
}

pub fn build(
  text_builder: TextCtx<'_, WindowContext>,
  client: Rect,
  props: FinderProps,
) -> Result<
  impl Fn(&mut Canvas<Window>) -> Result<(), TextError> + '_,
  TextError,
> {
  let remaining_width =
    (client.width() as f64 * props.remaining_ratio) as u32;

  let sentence_render =
    sentence::build(text_builder, client, props.sentence)?;

  Ok(move |mut canvas: &mut Canvas<Window>| {
    canvas.set_draw_color(Color::RGB(203, 193, 176));
    canvas
      .fill_rect(Rect::new(
        client.x(),
        client.y(),
        remaining_width,
        client.height(),
      ))
      .map_err(|e| TextError::RenderError(e))?;

    sentence_render(&mut canvas)?;
    Ok(())
  })
}
