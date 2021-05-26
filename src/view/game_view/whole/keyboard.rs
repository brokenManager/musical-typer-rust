use crate::view::renderer::{
  text::TextAlign, Component, Renderer, ViewResult,
};
use sdl2::{
  pixels::Color,
  rect::{Point, Rect},
};

const BLUE: Color = Color::RGB(64, 80, 180);
const ORANGE: Color = Color::RGB(209, 154, 29);
const GREEN: Color = Color::RGB(20, 76, 64);
const BACK: Color = Color::RGB(253, 243, 226);
const BLACK: Color = Color::RGB(0, 0, 0);
const GLAY: Color = Color::RGB(195, 195, 190);

#[derive(PartialEq)]
struct KeyCell {
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
  client: Rect,
}

impl KeyCell {
  fn bg_color(&self) -> Color {
    if self.is_highlighted {
      GREEN
    } else {
      BACK
    }
  }

  fn text_color(&self) -> Color {
    if self.is_pressed {
      ORANGE
    } else if self.is_highlighted {
      GLAY
    } else if self.key == 'f' || self.key == 'j' {
      BLUE
    } else {
      BLACK
    }
  }
}

impl Component for KeyCell {
  type Props = Self;

  fn update(&mut self, new_props: Self::Props) {
    *self = new_props;
  }

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    self != new_props
  }

  fn render(&self, canvas: &mut Renderer<'_, '_>) -> ViewResult {
    let border_dim = Rect::from_center(
      self.client.center(),
      self.client.width() - 5,
      self.client.height() - 5,
    );
    canvas.set_draw_color(self.bg_color());
    canvas.fill_rect(border_dim)?;
    canvas.set_draw_color(BLACK);
    canvas.draw_rect(border_dim)?;

    canvas.text(|s| {
      s.color(self.text_color())
        .text(&self.key.to_string().to_uppercase())
        .align(TextAlign::Center)
        .line_height(self.client.height())
        .pos(self.client.center())
    })?;
    Ok(())
  }
}

#[derive(PartialEq)]
pub struct KeyboardProps {
  pub pressed_keys: Vec<char>,
  pub highlighted_keys: Vec<char>,
}

pub struct Keyboard {
  props: KeyboardProps,
  cells: Vec<KeyCell>,
}

impl Keyboard {
  pub fn new(initial_props: KeyboardProps, client: Rect) -> Self {
    const CELL_ASPECT: f64 = 1.0;
    const KEY_CHARS_ROWS: &[&str] = &[
      "1234567890-^¥",
      "qwertyuiop@[",
      "asdfghjkl;:]",
      "zxcvbnm,./\\",
    ];

    let cell_height =
      client.height() as f64 / KEY_CHARS_ROWS.len() as f64;
    let cell_width = cell_height * CELL_ASPECT;

    let mut cells = vec![];

    for (y, key_chars_row) in KEY_CHARS_ROWS.iter().enumerate() {
      let y = y as f64;
      let row_amount = key_chars_row.len() as f64;
      let margin = client.width() as f64 - row_amount * cell_width;
      for (x, key_char) in key_chars_row.chars().enumerate() {
        let x = x as f64 + 1.0;
        let center = Point::new(
          (x * cell_width + client.x() as f64 + margin / 2.0) as i32,
          (y * cell_height + client.y() as f64 + cell_height / 2.0)
            as i32,
        );
        let key_cell_client = Rect::from_center(
          center,
          cell_width as u32,
          cell_height as u32,
        );
        cells.push(KeyCell {
          key: key_char,
          is_highlighted: initial_props
            .highlighted_keys
            .contains(&key_char),
          is_pressed: initial_props.pressed_keys.contains(&key_char),
          client: key_cell_client,
        });
      }
    }

    Self {
      cells,
      props: initial_props,
    }
  }
}

impl Component for Keyboard {
  type Props = KeyboardProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: KeyboardProps) {
    self.props = new_props;
  }

  fn render(&self, ctx: &mut Renderer<'_, '_>) -> ViewResult {
    for cell in &self.cells {
      cell.render(ctx)?;
    }
    Ok(())
  }
}
