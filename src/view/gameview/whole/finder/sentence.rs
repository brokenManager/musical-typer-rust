use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::text::{TextAlign, TextCtx, TextError},
};
use sdl2::{
  pixels::Color,
  rect::{Point, Rect},
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
    const JAPANESE_HEIGHT: u32 = 20;
    let half_x = client.width() / 2;

    const ROMAN_HEIGHT: u32 = 50;

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
        .line_height(JAPANESE_HEIGHT)
        .align(TextAlign::Left)
        .build()?
    };

    let will_input_yomigana_text;
    let inputted_yomigana_text;
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.yomiagana();

      will_input_yomigana_text = {
        text_builder
          .borrow_mut()
          .color(Color::RGB(0, 0, 0))
          .text(&will_input)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Left)
          .build()?
      };
      inputted_yomigana_text = {
        text_builder
          .borrow_mut()
          .color(Color::RGB(80, 80, 80))
          .text(&inputted)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Right)
          .build()?
      };
    }

    let will_input_roman_text;
    let inputted_roman_text;
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.roman();

      will_input_roman_text = {
        text_builder
          .borrow_mut()
          .color(Color::RGB(0, 0, 0))
          .text(&will_input)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Left)
          .build()?
      };
      inputted_roman_text = {
        text_builder
          .borrow_mut()
          .color(Color::RGB(80, 80, 80))
          .text(&inputted)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Right)
          .build()?
      };
    }

    Ok(Box::new(move |mut canvas| {
      will_input_japanese_text.render_with(
        &mut canvas,
        |(w, _)| {
          Point::new(
            (half_x as f64 - normalized_inputted * w as f64) as i32,
            client.y(),
          )
        },
      )?;
      will_input_yomigana_text.render(
        &mut canvas,
        Point::new(
          half_x as i32 + 5,
          client.y() + client.height() as i32
            - 2 * ROMAN_HEIGHT as i32,
        ),
      )?;
      inputted_yomigana_text.render(
        &mut canvas,
        Point::new(
          half_x as i32 - 5,
          client.y() + client.height() as i32
            - 2 * ROMAN_HEIGHT as i32,
        ),
      )?;
      will_input_roman_text.render(
        &mut canvas,
        Point::new(
          half_x as i32 + 5,
          client.y() + client.height() as i32 - ROMAN_HEIGHT as i32,
        ),
      )?;
      inputted_roman_text.render(
        &mut canvas,
        Point::new(
          half_x as i32 - 5,
          client.y() + client.height() as i32 - ROMAN_HEIGHT as i32,
        ),
      )?;
      Ok(())
    }))
  } else {
    Ok(Box::new(|_canvas| Ok(())))
  }
}
