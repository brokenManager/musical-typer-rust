use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::text::{TextCtx, TextError},
};
use sdl2::{
  pixels::Color,
  rect::Rect,
  render::Canvas,
  video::{Window, WindowContext},
};

pub fn build(
  text_builder: TextCtx<'_, WindowContext>,
  client: Rect,
  sentence: Option<Sentence>,
) -> Result<
  Box<dyn Fn(&mut Canvas<Window>) -> Result<(), TextError> + '_>,
  TextError,
> {
  if let Some(sentence) = sentence {
    const JAPANESE_GLYPH_WIDTH: u32 = 20;
    const JAPANESE_HEIGHT: u32 = 80;
    let half_x = client.width() / 2;

    const ROMAN_GLYPH_WIDTH: u32 = 20;
    const ROMAN_HEIGHT: u32 = 20;

    let roman = sentence.roman();
    let full_roman_len =
      roman.will_input.len() + roman.inputted.len();
    let normalized_inputted =
      roman.inputted.len() as f64 / full_roman_len as f64;

    let will_input_japanese = sentence.origin().to_owned();
    let will_input_japanese_text = {
      text_builder
        .borrow_mut()
        .color(Color::RGB(0, 0, 0))
        .text(&will_input_japanese)
        .build()?
    };

    let TypingStr {
      will_input,
      inputted,
    } = sentence.roman();

    let will_input_text = {
      text_builder
        .borrow_mut()
        .color(Color::RGB(0, 0, 0))
        .text(&will_input)
        .build()?
    };

    let inputted_text = {
      text_builder
        .borrow_mut()
        .color(Color::RGB(80, 80, 80))
        .text(&inputted)
        .build()?
    };

    Ok(Box::new(move |mut canvas| {
      will_input_japanese_text.render(
        &mut canvas,
        Rect::new(
          (half_x as f64
            - (normalized_inputted
              * will_input_japanese.len() as f64)
              * JAPANESE_GLYPH_WIDTH as f64) as i32,
          client.y(),
          will_input_japanese.len() as u32 * JAPANESE_GLYPH_WIDTH,
          JAPANESE_HEIGHT,
        ),
      )?;
      will_input_text.render(
        &mut canvas,
        Rect::new(
          half_x as i32,
          client.y() + JAPANESE_HEIGHT as i32,
          will_input.len() as u32 * ROMAN_GLYPH_WIDTH,
          ROMAN_HEIGHT + ROMAN_HEIGHT,
        ),
      )?;
      inputted_text.render(
        &mut canvas,
        Rect::new(
          half_x as i32
            - (inputted.len() + 1) as i32 * ROMAN_GLYPH_WIDTH as i32,
          client.y() + JAPANESE_HEIGHT as i32,
          inputted.len() as u32 * ROMAN_GLYPH_WIDTH,
          ROMAN_HEIGHT + ROMAN_HEIGHT,
        ),
      )?;
      Ok(())
    }))
  } else {
    Ok(Box::new(|_canvas| Ok(())))
  }
}
