use crate::view::renderer::{text::TextAlign, RenderCtx, ViewResult};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

const CELL_WIDTH: u32 = 60;
const CELL_HEIGHT: u32 = 70;

fn key_cell(
  center: Point,
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
) -> impl Fn(RenderCtx) -> ViewResult {
  move |ctx: RenderCtx| -> ViewResult {
    const ORANGE: Color = Color::RGB(209, 154, 29);
    const GREEN: Color = Color::RGB(20, 76, 64);
    const BACK: Color = Color::RGB(253, 243, 226);
    const BLACK: Color = Color::RGB(0, 0, 0);
    let client = Rect::from_center(center, CELL_WIDTH, CELL_HEIGHT);
    ctx.borrow_mut().set_draw_color(if is_highlighted {
      GREEN
    } else {
      BACK
    });
    ctx.borrow_mut().fill_rect(client)?;
    ctx.borrow_mut().set_draw_color(BLACK);
    ctx.borrow_mut().draw_rect(Rect::from_center(
      center,
      CELL_WIDTH,
      CELL_HEIGHT,
    ))?;
    ctx.borrow_mut().text(|s| {
      s.color(if is_pressed {
        ORANGE
      } else if is_highlighted {
        BACK
      } else {
        BLACK
      })
      .text(&key.to_string())
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
        key_cell(
          center,
          key_char,
          highlighted_keys.contains(&key_char),
          pressed_keys.contains(&key_char),
        )(ctx.clone())?;
        x += 1;
      }
      y += 1;
    }
    Ok(())
  }
}
