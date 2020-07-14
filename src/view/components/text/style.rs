use sdl2::{
  pixels::Color,
  rect::{Point, Rect},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TextStyle {
  text: String,
  color: Color,
  line_height: u32,
  align: TextAlign,
  pos: Point,
  opacity: u8,
}

impl TextStyle {
  pub fn new() -> Self {
    TextStyle {
      text: "".into(),
      color: Color::RGB(0, 0, 0),
      line_height: 20,
      align: TextAlign::Left,
      pos: Point::new(0, 0),
      opacity: u8::max_value(),
    }
  }

  pub fn text(mut self, new_text: &str) -> Self {
    if new_text == "" {
      self.text = String::from(" ");
    } else {
      self.text = new_text.into();
    }

    self
  }

  pub fn color(mut self, new_color: Color) -> Self {
    self.color = new_color;
    self
  }

  pub fn line_height(mut self, new_line_height: u32) -> Self {
    self.line_height = new_line_height;
    self
  }

  pub fn align(mut self, new_align: TextAlign) -> Self {
    self.align = new_align;
    self
  }

  pub fn pos(mut self, new_pos: Point) -> Self {
    self.pos = new_pos;
    self
  }

  pub fn opacity(mut self, new_opacity: u8) -> Self {
    self.opacity = new_opacity;
    self
  }

  pub fn to_rect(&self, aspect: f64) -> Rect {
    let (w, h) =
      ((aspect * self.line_height as f64) as u32, self.line_height);
    use TextAlign::*;

    match self.align {
      Left => Rect::new(self.pos.x(), self.pos.y(), w, h),
      Center => Rect::from_center(self.pos, w, h),
      Right => Rect::new(self.pos.x() - w as i32, self.pos.y(), w, h),
    }
  }
}
