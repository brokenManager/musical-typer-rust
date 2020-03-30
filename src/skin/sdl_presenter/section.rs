use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::text::TextBuilder;

use crate::exp::string_to_input::StringToInput;

pub struct Section<'a> {
  to_input: &'a StringToInput,
  remaining_ratio: f64,
}

impl<'a> Section<'a> {
  pub fn new(
    to_input: &'a StringToInput,
    remaining_ratio: f64,
  ) -> Section<'a> {
    Section {
      to_input,
      remaining_ratio: remaining_ratio.max(0.).min(1.),
    }
  }

  pub fn draw<T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
    offset: Rect,
  ) -> Result<(), String> {
    let remaining_width =
      (offset.width() as f64 * self.remaining_ratio) as u32;
    canvas.set_draw_color(Color::RGB(203, 193, 176));
    canvas.fill_rect(Rect::new(
      offset.x(),
      offset.y(),
      remaining_width,
      offset.height(),
    ))?;

    const JAPANESE_GLYPH_WIDTH: u32 = 20;
    const JAPANESE_HEIGHT: u32 = 80;
    let half_x = offset.width() / 2;

    let will_input_japanese = self.to_input.origin();
    text_builder
      .color(Color::RGB(0, 0, 0))
      .text(will_input_japanese)
      .build()
      .render(
        &mut canvas,
        Rect::new(
          half_x as i32,
          offset.y(),
          will_input_japanese.len() as u32 * JAPANESE_GLYPH_WIDTH,
          JAPANESE_HEIGHT,
        ),
      )?;

    const ROMAN_GLYPH_WIDTH: u32 = 20;
    const ROMAN_HEIGHT: u32 = 20;
    let will_input_roman = self.to_input.will_input();
    text_builder
      .color(Color::RGB(0, 0, 0))
      .text(will_input_roman.as_str())
      .build()
      .render(
        &mut canvas,
        Rect::new(
          half_x as i32,
          offset.y() + JAPANESE_HEIGHT as i32,
          will_input_japanese.len() as u32 * ROMAN_GLYPH_WIDTH,
          ROMAN_HEIGHT + ROMAN_HEIGHT,
        ),
      )?;

    Ok(())
  }
}
