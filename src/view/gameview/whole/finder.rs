use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::{
    renderer::{text::TextAlign, RenderCtx},
    ViewError,
  },
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

  pub fn draw<'texture>(
    &self,
    ctx: RenderCtx<'_, 'texture>,
    offset: Rect,
  ) -> Result<(), ViewError> {
    let remaining_width =
      (offset.width() as f64 * self.remaining_ratio) as u32;
    ctx.borrow_mut().set_draw_color(Color::RGB(203, 193, 176));
    ctx.borrow_mut().fill_rect(Rect::new(
      offset.x(),
      offset.y(),
      remaining_width,
      offset.height(),
    ))?;

    const JAPANESE_HEIGHT: u32 = 80;
    let half_x = offset.width() / 2;

    if let Some(sentence) = self.sentence {
      let will_input_japanese = sentence.origin();
      ctx.borrow_mut().text(|s| {
        s.color(Color::RGB(0, 0, 0))
          .text(will_input_japanese)
          .line_height(JAPANESE_HEIGHT)
          .align(TextAlign::Left)
          .pos(Point::new(0, 0))
      })?;

      {
        const ROMAN_HEIGHT: u32 = 40;
        let TypingStr {
          will_input,
          inputted,
        } = sentence.roman();
        let will_input = will_input.as_str();
        let inputted = inputted.as_str();

        ctx.borrow_mut().text(|s| {
          s.color(Color::RGB(0, 0, 0))
            .text(will_input)
            .line_height(ROMAN_HEIGHT)
            .align(TextAlign::Left)
            .pos(Point::new(
              half_x as i32 + 5,
              offset.y() + JAPANESE_HEIGHT as i32,
            ))
        })?;

        ctx.borrow_mut().text(|s| {
          s.color(Color::RGB(80, 80, 80))
            .text(inputted)
            .line_height(ROMAN_HEIGHT)
            .align(TextAlign::Right)
            .pos(Point::new(
              half_x as i32 - 5,
              offset.y() + JAPANESE_HEIGHT as i32,
            ))
        })?;
      }
    }
    Ok(())
  }
}
