use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::super::text::{TextBuilder, TextError};

use crate::model::exp::sentence::{Sentence, TypingStr};

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

  pub fn draw<T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
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

    const JAPANESE_GLYPH_WIDTH: u32 = 20;
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
        .build()?
        .render(
          &mut canvas,
          Rect::new(
            (half_x as f64
              - (normalized_inputted
                * will_input_japanese.len() as f64)
                * JAPANESE_GLYPH_WIDTH as f64) as i32,
            offset.y(),
            will_input_japanese.len() as u32 * JAPANESE_GLYPH_WIDTH,
            JAPANESE_HEIGHT,
          ),
        )?;

      {
        const ROMAN_GLYPH_WIDTH: u32 = 20;
        const ROMAN_HEIGHT: u32 = 20;
        let TypingStr {
          will_input,
          inputted,
        } = sentence.roman();
        let will_input = will_input.as_str();
        let inputted = inputted.as_str();

        text_builder
          .color(Color::RGB(0, 0, 0))
          .text(will_input)
          .build()?
          .render(
            &mut canvas,
            Rect::new(
              half_x as i32,
              offset.y() + JAPANESE_HEIGHT as i32,
              will_input.len() as u32 * ROMAN_GLYPH_WIDTH,
              ROMAN_HEIGHT + ROMAN_HEIGHT,
            ),
          )?;

        text_builder
          .color(Color::RGB(80, 80, 80))
          .text(inputted)
          .build()?
          .render(
            &mut canvas,
            Rect::new(
              half_x as i32
                - (inputted.len() + 1) as i32
                  * ROMAN_GLYPH_WIDTH as i32,
              offset.y() + JAPANESE_HEIGHT as i32,
              inputted.len() as u32 * ROMAN_GLYPH_WIDTH,
              ROMAN_HEIGHT + ROMAN_HEIGHT,
            ),
          )?;
      }
    }
    Ok(())
  }
}
