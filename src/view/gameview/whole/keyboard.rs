use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};

const CELL_WIDTH: u32 = 60;
const CELL_HEIGHT: u32 = 70;

pub struct KeyCell {
  center: Point,
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
}

impl KeyCell {
  fn new(
    center: Point,
    key: char,
    is_highlighted: bool,
    is_pressed: bool,
  ) -> Self {
    KeyCell {
      center,
      key,
      is_highlighted,
      is_pressed,
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: &mut TextBuilder<'a, U>,
  ) -> Result<(), TextError> {
    const ORANGE: Color = Color::RGB(209, 154, 29);
    const GREEN: Color = Color::RGB(20, 76, 64);
    const BACK: Color = Color::RGB(253, 243, 226);
    const BLACK: Color = Color::RGB(0, 0, 0);
    let client =
      Rect::from_center(self.center, CELL_WIDTH, CELL_HEIGHT);
    canvas.set_draw_color(if self.is_highlighted {
      GREEN
    } else {
      BACK
    });
    canvas
      .fill_rect(client)
      .map_err(|e| TextError::RenderError(e))?;
    canvas.set_draw_color(BLACK);
    canvas
      .draw_rect(Rect::from_center(
        self.center,
        CELL_WIDTH,
        CELL_HEIGHT,
      ))
      .map_err(|e| TextError::RenderError(e))?;
    text_builder
      .color(if self.is_pressed {
        ORANGE
      } else if self.is_highlighted {
        BACK
      } else {
        BLACK
      })
      .text(&self.key.to_string())
      .build()?
      .render(&mut canvas, client)?;
    Ok(())
  }
}

pub struct Keyboard {
  pressed_keys: Vec<char>,
  highlighted_keys: Vec<char>,
}

impl Keyboard {
  pub fn new(
    pressed_keys: &[char],
    highlighted_keys: &[char],
  ) -> Self {
    Keyboard {
      pressed_keys: pressed_keys.to_owned(),
      highlighted_keys: highlighted_keys.to_owned(),
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    text_builder: &mut TextBuilder<'a, U>,
    offset: Rect,
  ) -> Result<(), TextError> {
    let key_chars_rows =
      ["1234567890-", "qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let mut y = 0;
    for key_chars_row in key_chars_rows.iter() {
      let mut x = 0;
      for key_char in key_chars_row.chars() {
        let width = key_chars_row.len() as u32 - 1;
        let center = Point::new(
          x * CELL_WIDTH as i32
            + offset.x()
            + (offset.width() - width * CELL_WIDTH) as i32 / 2,
          y * CELL_HEIGHT as i32
            + offset.y()
            + CELL_HEIGHT as i32 * 2 / 3,
        );
        let cell = KeyCell::new(
          center,
          key_char,
          self.highlighted_keys.contains(&key_char),
          self.pressed_keys.contains(&key_char),
        );
        cell.draw(&mut canvas, text_builder)?;
        x += 1;
      }
      y += 1;
    }
    Ok(())
  }
}
