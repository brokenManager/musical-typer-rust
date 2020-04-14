use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

use super::super::super::text::{TextBuilder, TextError};

const CELL_ASPECT: f64 = 55.0 / 70.0;

pub struct KeyCell {
  client: Rect,
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
}

impl KeyCell {
  pub fn draw<'a, T: RenderTarget, U>(
    self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
  ) -> Result<(), TextError> {
    const ORANGE: Color = Color::RGB(209, 154, 29);
    const GREEN: Color = Color::RGB(20, 76, 64);
    const BACK: Color = Color::RGB(253, 243, 226);
    const BLACK: Color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(if self.is_highlighted {
      GREEN
    } else {
      BACK
    });
    canvas
      .fill_rect(self.client)
      .map_err(|e| TextError::RenderError(e))?;
    canvas.set_draw_color(BLACK);
    canvas
      .draw_rect(self.client)
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
      .render(&mut canvas, self.client)?;
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
    text_builder: TextBuilder<'a, U>,
    client: Rect,
  ) -> Result<(), TextError> {
    let key_chars_rows =
      ["1234567890-", "qwertyuiop", "asdfghjkl", "zxcvbnm"];

    let cell_height =
      client.height() as f64 / key_chars_rows.len() as f64;
    let cell_width = cell_height * CELL_ASPECT;

    for (y, key_chars_row) in key_chars_rows.iter().enumerate() {
      for (x, key_char) in key_chars_row.chars().enumerate() {
        let width = key_chars_row.len() as u32 - 1;
        let center = Point::new(
          (x as f64 * cell_width
            + client.x() as f64
            + (client.width() as f64 - width as f64 * cell_width)
              / 2.0) as i32,
          (y as f64 * cell_height
            + client.y() as f64
            + cell_height / 2.0) as i32,
        );
        KeyCell {
          client: Rect::from_center(
            center,
            cell_width as u32,
            cell_height as u32,
          ),
          key: key_char,
          is_highlighted: self.highlighted_keys.contains(&key_char),
          is_pressed: self.pressed_keys.contains(&key_char),
        }
        .draw(&mut canvas, text_builder.clone())?;
      }
    }
    Ok(())
  }
}
