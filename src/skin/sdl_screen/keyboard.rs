use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

use super::text::TextBuilder;

const CELL_WIDTH: u32 = 65;
const CELL_HEIGHT: u32 = 85;

pub struct KeyCell {
  center: Point,
  key: char,
  is_pressed: bool,
}

impl KeyCell {
  fn new(center: Point, key: char, is_pressed: bool) -> KeyCell {
    KeyCell {
      center,
      key,
      is_pressed,
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    self,
    mut canvas: &mut Canvas<T>,
    mut text_builder: TextBuilder<'a, U>,
  ) -> Result<(), String> {
    const GREEN: Color = Color::RGB(20, 76, 64);
    const BACK: Color = Color::RGB(253, 243, 226);
    const BLACK: Color = Color::RGB(0, 0, 0);
    let client = Rect::from_center(self.center, CELL_WIDTH, CELL_HEIGHT);
    canvas.set_draw_color(if self.is_pressed { GREEN } else { BACK });
    canvas.fill_rect(client)?;
    canvas.set_draw_color(BLACK);
    canvas.draw_rect(Rect::from_center(self.center, CELL_WIDTH, CELL_HEIGHT))?;
    text_builder
      .color(if self.is_pressed { BACK } else { BLACK })
      .text(&self.key.to_string())
      .build()
      .render(&mut canvas, client)?;
    Ok(())
  }
}

pub struct Keyboard {
  pressed_keys: Vec<char>,
}

impl Keyboard {
  pub fn new(pressed_keys: &[char]) -> Keyboard {
    Keyboard {
      pressed_keys: pressed_keys.to_owned(),
    }
  }

  pub fn draw<'a, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    text_builder: TextBuilder<'a, U>,
    offset: Rect,
  ) -> Result<(), String> {
    let key_chars_rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let mut y = 0;
    for key_chars_row in key_chars_rows.iter() {
      let mut x = 0;
      for key_char in key_chars_row.chars() {
        let width = key_chars_row.len() as u32 - 1;
        let center = Point::new(
          x * CELL_WIDTH as i32 + offset.x() + (offset.width() - width * CELL_WIDTH) as i32 / 2,
          y * CELL_HEIGHT as i32 + offset.y() + CELL_HEIGHT as i32 * 2 / 3,
        );
        let cell = KeyCell::new(center, key_char, self.pressed_keys.contains(&key_char));
        {
          cell.draw(&mut canvas, text_builder.clone())?;
        }
        x += 1;
      }
      y += 1;
    }
    Ok(())
  }
}
