use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{
  render::{Canvas, RenderTarget},
  video::{Window, WindowContext},
};

use crate::view::text::{TextAlign, TextCtx, TextError};

const CELL_ASPECT: f64 = 55.0 / 70.0;

struct KeyCell {
  client: Rect,
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
}

impl KeyCell {
  pub fn draw<'a, T: RenderTarget, U>(
    &self,
    mut canvas: &mut Canvas<T>,
    text_builder: TextCtx<'a, U>,
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
      .borrow_mut()
      .color(if self.is_pressed {
        ORANGE
      } else if self.is_highlighted {
        BACK
      } else {
        BLACK
      })
      .text(&self.key.to_string())
      .line_height(self.client.height())
      .align(TextAlign::Center)
      .build()?
      .render(&mut canvas, self.client.center())?;
    Ok(())
  }
}

pub struct KeyboardProps {
  pub pressed_keys: Vec<char>,
  pub highlighted_keys: Vec<char>,
}

pub fn build(
  text_builder: TextCtx<'_, WindowContext>,
  client: Rect,
  props: KeyboardProps,
) -> Result<
  impl Fn(&mut Canvas<Window>) -> Result<(), TextError> + '_,
  TextError,
> {
  let key_chars_rows =
    ["1234567890-", "qwertyuiop", "asdfghjkl", "zxcvbnm"];

  let cell_height =
    client.height() as f64 / key_chars_rows.len() as f64;
  let cell_width = cell_height * CELL_ASPECT;
  let mut key_cells = vec![];

  for (y, key_chars_row) in key_chars_rows.iter().enumerate() {
    for (x, key_char) in key_chars_row.chars().enumerate() {
      let width = key_chars_row.len() as u32 - 1;
      let center = Point::new(
        (x as f64 * cell_width
          + client.x() as f64
          + (client.width() as f64 - width as f64 * cell_width) / 2.0)
          as i32,
        (y as f64 * cell_height
          + client.y() as f64
          + cell_height / 2.0) as i32,
      );
      key_cells.push(KeyCell {
        client: Rect::from_center(
          center,
          cell_width as u32,
          cell_height as u32,
        ),
        key: key_char,
        is_highlighted: props.highlighted_keys.contains(&key_char),
        is_pressed: props.pressed_keys.contains(&key_char),
      });
    }
  }
  Ok(move |mut canvas: &mut Canvas<Window>| {
    for key_cell in key_cells.iter() {
      key_cell.draw(&mut canvas, text_builder.clone())?;
    }
    Ok(())
  })
}
