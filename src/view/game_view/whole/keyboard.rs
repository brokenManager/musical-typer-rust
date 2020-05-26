use crate::view::renderer::{text::TextAlign, RenderCtx, ViewResult};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

const CELL_ASPECT: f64 = 1.0;

fn key_cell(
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
) -> impl Fn(RenderCtx, Rect) -> ViewResult {
  move |ctx: RenderCtx, client: Rect| -> ViewResult {
    const BLUE: Color = Color::RGB(64, 80, 180);
    const ORANGE: Color = Color::RGB(209, 154, 29);
    const GREEN: Color = Color::RGB(20, 76, 64);
    const BACK: Color = Color::RGB(253, 243, 226);
    const BLACK: Color = Color::RGB(0, 0, 0);
    const GLAY: Color = Color::RGB(195, 195, 190);

    let border_dim = Rect::from_center(
      client.center(),
      client.width() - 5,
      client.height() - 5,
    );
    ctx.borrow_mut().set_draw_color(if is_highlighted {
      GREEN
    } else {
      BACK
    });
    ctx.borrow_mut().fill_rect(border_dim)?;
    ctx.borrow_mut().set_draw_color(BLACK);
    ctx.borrow_mut().draw_rect(border_dim)?;

    ctx.borrow_mut().text(|s| {
      s.color(if is_pressed {
        ORANGE
      } else if is_highlighted {
        GLAY
      } else if key == 'f' || key == 'j' {
        BLUE
      } else {
        BLACK
      })
      .text(&key.to_string().to_uppercase())
      .align(TextAlign::Center)
      .line_height(client.height())
      .pos(client.center())
    })?;
    Ok(())
  }
}

pub fn keyboard<
  'renderer,
  'pressed: 'renderer,
  'highlighted: 'renderer,
>(
  pressed_keys: &'pressed [char],
  highlighted_keys: &'highlighted [char],
) -> impl Fn(RenderCtx, Rect) -> ViewResult + 'renderer {
  move |ctx: RenderCtx, offset: Rect| -> ViewResult {
    let key_chars_rows = [
      "1234567890-^¥",
      "qwertyuiop@[",
      "asdfghjkl;:]",
      "zxcvbnm,./\\",
    ];
    let cell_height =
      offset.height() as f64 / key_chars_rows.len() as f64;
    let cell_width = cell_height * CELL_ASPECT;

    for (y, key_chars_row) in key_chars_rows.iter().enumerate() {
      let y = y as f64;
      let row_amount = key_chars_row.len() as f64;
      let margin = offset.width() as f64 - row_amount * cell_width;
      for (x, key_char) in key_chars_row.chars().enumerate() {
        let x = x as f64 + 1.0;
        let center = Point::new(
          (x * cell_width + offset.x() as f64 + margin / 2.0) as i32,
          (y * cell_height + offset.y() as f64 + cell_height / 2.0)
            as i32,
        );
        let key_cell_client = Rect::from_center(
          center,
          cell_width as u32,
          cell_height as u32,
        );
        key_cell(
          key_char,
          highlighted_keys.contains(&key_char),
          pressed_keys.contains(&key_char),
        )(ctx.clone(), key_cell_client)?;
      }
    }
    Ok(())
  }
}
