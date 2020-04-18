use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};

use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::text::TextAlign,
};

pub struct Finder<'a> {
  sentence: &'a Option<Sentence>,
  remaining_ratio: f64,
}

impl<'a> Finder<'a> {
  pub fn new(
    sentence: &'a Option<Sentence>,
    remaining_ratio: f64,
  ) -> Self {
    Finder {
      sentence,
      remaining_ratio: remaining_ratio.max(0.).min(1.),
    }
  }

  pub fn draw<'builder, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    text_builder: &mut TextBuilder<'builder, U>,
    offset: Rect,
  ) -> Result<(), TextError> {
    let remaining_width =
      (offset.width() as f64 * self.remaining_ratio) as u32;
    canvas.set_draw_color(Color::RGB(203, 193, 176));
    canvas
      .fill_rect(Rect::new(
        offset.x(),
        offset.y(),
        remaining_width,
        offset.height(),
      ))
      .map_err(|e| TextError::RenderError(e))?;

    const JAPANESE_HEIGHT: u32 = 80;
    let half_x = offset.width() / 2;

    if let Some(sentence) = self.sentence {
      let roman = sentence.roman();
      let full_roman_len =
        roman.will_input.len() + roman.inputted.len();
      let normalized_inputted =
        roman.inputted.len() as f64 / full_roman_len as f64;

      let will_input_japanese = sentence.origin();
      text_builder
        .color(Color::RGB(0, 0, 0))
        .text(will_input_japanese)
        .line_height(JAPANESE_HEIGHT)
        .align(TextAlign::Left)
        .build()?
        .render_with(&mut canvas, |(width, _)| {
          Point::new(
            (half_x as f64 - normalized_inputted * width as f64)
              as i32,
            offset.y(),
          )
        })?;

      {
        const ROMAN_HEIGHT: u32 = 40;
        let TypingStr {
          will_input,
          inputted,
        } = sentence.roman();
        let will_input = will_input.as_str();
        let inputted = inputted.as_str();

        text_builder
          .color(Color::RGB(0, 0, 0))
          .text(will_input)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Left)
          .build()?
          .render(
            &mut canvas,
            Point::new(
              half_x as i32 + 5,
              offset.y() + JAPANESE_HEIGHT as i32,
            ),
          )?;

        text_builder
          .color(Color::RGB(80, 80, 80))
          .text(inputted)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Right)
          .build()?
          .render(
            &mut canvas,
            Point::new(
              half_x as i32 - 5,
              offset.y() + JAPANESE_HEIGHT as i32,
            ),
          )?;
      }
    }
    Ok(())
  }
}
